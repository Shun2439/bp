{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = with pkgs; [
    gdb
  ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  enterShell = ''
    exec fish
  '';
}

