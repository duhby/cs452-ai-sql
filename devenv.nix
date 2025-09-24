{ pkgs, lib, config, ... }: {
  languages.rust.enable = true;

  packages = [
    pkgs.sqlx-cli
    pkgs.openssl
  ];

  services.postgres = {
    enable = true;
    package = pkgs.postgresql_17;
  };
}
