use async_openai::{
    Client as OAIClient,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs,
    },
};
use regex::Regex;
use std::error::Error;

const INIT_MIGRATION: &str = include_str!("../migrations/20250924192756_init.up.sql");

fn extract_sql_block(message: &str) -> Option<String> {
    let re = Regex::new(r"```sql\s*\n([\s\S]*?)```").unwrap();
    re.captures(message)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
}

fn get_request(
    messages: &[ChatCompletionRequestMessage],
) -> Result<CreateChatCompletionRequest, Box<dyn Error>> {
    Ok(CreateChatCompletionRequestArgs::default()
        .max_completion_tokens(2048u32)
        // .model("gpt-5-nano-2025-08-07")
        .model("gpt-4.1-mini-2025-04-14")
        .messages(messages)
        .build()?)
}

fn get_user_message(content: &str) -> Result<ChatCompletionRequestMessage, Box<dyn Error>> {
    Ok(ChatCompletionRequestUserMessageArgs::default()
        .content(content)
        .build()?
        .into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let db_uri = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Connecting to database at {}", db_uri);
    let db_pool = sqlx::PgPool::connect(&db_uri)
        .await
        .expect("failed to connect to database");

    println!("Applying database migrations");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("failed to apply migrations");

    // Used by OAIClient's default config
    dotenvy::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    let oai_client = OAIClient::new();

    let system_message: ChatCompletionRequestMessage =
        ChatCompletionRequestSystemMessageArgs::default()
            .content(format!(
                r#"
                    Here is a postgres 17 database schema.
                    {}
                    Write a select statement that answers the following question.
                    Do not give an explanation.
                    Do not write anything but the raw query.
                    If you select type_, then cast it to text like type_::text.
                "#,
                INIT_MIGRATION,
            ))
            .build()?
            .into();

    let questions = [
        "How many users are in the database?",
        "How many files does each member have?",
        "What is the most common file type?",
        "Who has uploaded the most files?",
        "Which tags have no children?",
        "Which members have at least one file with more than one tag?",
    ]
    .iter_mut()
    .map(|q| q.to_string())
    .collect::<Vec<_>>();

    for question in &questions {
        println!("---\nQuestion: {}", question);
        let request = get_request(&[system_message.clone(), get_user_message(question)?])?;

        let openai_response = oai_client.chat().create(request).await?;
        let query = openai_response.choices[0]
            .clone()
            .message
            .content
            .ok_or("API returned no content")?;

        let query = extract_sql_block(&query).unwrap_or(query);

        println!("Query: {}", query);

        let db_response = sqlx::query(&query).fetch_all(&db_pool).await?;
        println!("DB Response: {:?}", db_response);

        let request = get_request(&[get_user_message(&format!(
            "I asked a question: `{}`. The SQL query was: `{}`. The SQL response is \n{:?}\n. Give a 1 sentence answer to the question given the data. Do not explain it.",
            question, query, db_response,
        ))?])?;

        let openai_response = oai_client.chat().create(request).await?;
        let answer = openai_response.choices[0]
            .clone()
            .message
            .content
            .ok_or("API returned no content")?;
        println!("Answer: {}", answer);
    }

    Ok(())
}
