{ pkgs, ... }:
{
  packages = with pkgs; [
    meson
    ninja
    (python3.withPackages (ps: with ps; [ cffi matplotlib numpy ]))
  ];

  languages.rust.enable = true;
}
