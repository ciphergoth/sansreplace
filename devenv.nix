{ pkgs, ... }:
{
  packages = with pkgs; [
    meson
    ninja
    # setuptools is needed for cffi to compile the C++ bindings
    (python3.withPackages (ps: with ps; [ cffi matplotlib numpy setuptools ]))
  ];

  languages.rust.enable = true;
}
