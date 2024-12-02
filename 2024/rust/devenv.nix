{ pkgs, lib, ... }:

{
  cachix.enable = false;

  dotenv.enable = true;
  # https://devenv.sh/packages/
  packages =
    with pkgs;
    [
      cargo-watch
      cargo-generate
      cargo-nextest
      just
      rustup
    ]
    ++ lib.optionals pkgs.stdenv.isDarwin [
      # darwin.apple_sdk.frameworks.CoreFoundation
      # darwin.apple_sdk.frameworks.Security
      darwin.apple_sdk.frameworks.SystemConfiguration
      # libiconv
      # darwin.apple_sdk.Libsystem
      # darwin.apple_sdk.frameworks.AppKit
      # darwin.apple_sdk.frameworks.Cocoa
      # darwin.apple_sdk.frameworks.CoreServices
      # darwin.apple_sdk.frameworks.Security
      # darwin.apple_sdk.frameworks.SystemConfiguration
      # darwin.apple_sdk.frameworks.WebKit
      # darwin.apple_sdk.frameworks.Foundation
      # darwin.apple_sdk.frameworks.SwiftUI
      # darwin.apple_sdk.frameworks.UIFoundation
      # darwin.libobjc
    ];

  scripts.hello.exec = ''
    echo "Welcome to Advent of Code 2024! 🎄 (Rust edition 🦀)"
  '';

  enterShell = ''
    hello
  '';

  languages = {
    rust = {
      enable = true;
      channel = "nightly";
      targets =
        [
          # "aarch64-linux-android"
        ]
        ++ lib.optionals pkgs.stdenv.isDarwin [
          "aarch64-apple-darwin"
        ];
      components = [
        "rustc"
        "cargo"
        "clippy"
        "rustfmt"
        "rust-analyzer"
      ];

      # mold.enable = true;
    };
  };
  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  # enterTest = ''
  #   echo "Running tests"
  #   git --version | grep --color=auto "${pkgs.git.version}"
  # '';

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
