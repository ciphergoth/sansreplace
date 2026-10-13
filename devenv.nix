{ pkgs, ... }:
let
  # The latexminted 0.6.0 bundled with minted 3.7.0 fails on Python 3.14
  # (https://github.com/NixOS/nixpkgs/issues/542483). This applies the fix from
  # https://github.com/NixOS/nixpkgs/pull/569745, loading a patched copy of
  # latexminted into minted's bin container, which we use with its unpatched tex
  # container; remove it once nixpkgs includes that PR.
  minted = pkgs.texlive.pkgs.minted;
  mintedPatch = pkgs.fetchpatch2 {
    name = "minted-python314-compat.patch";
    url = "https://github.com/gpoore/minted/commit/2a7b3a48e47b834bbc4a616679a0ef9710896a16.diff";
    hash = "sha256-JcaQjSDqBQFsWLNRaOSNxCvPNa2HktfuzjsMKAWUPl8=";
    includes = [ "python/latexminted/cmdline.py" ];
  };
  patchedMintedBin = minted.out.overrideAttrs (old: {
    nativeBuildInputs = (old.nativeBuildInputs or [ ]) ++ [ pkgs.unzip ];
    buildCommand = old.buildCommand + ''
      latexmintedLib="$out"/share/minted/python
      mkdir -p "$latexmintedLib"
      unzip -q "$scriptsFolder"/latexminted-0.6.0-py3-none-any.whl 'latexminted/*' -d "$latexmintedLib"
      sed -i 's/\r$//' "$latexmintedLib"/latexminted/cmdline.py
      patch -p2 -d "$latexmintedLib" -i "${mintedPatch}"
      substituteInPlace "$out"/bin/latexminted \
        --replace-fail "from latexminted.cmdline import main" \
          "sys.path.insert(0, '$latexmintedLib')"$'\n'"from latexminted.cmdline import main"
    '';
  });
in
{
  packages = with pkgs; [
    meson
    ninja
    (python3.withPackages (ps: with ps; [ cffi matplotlib numpy requests ]))
    biber
    (texliveMedium.withPackages (
      ps: with ps; [
        abstract
        bigfoot
        biblatex
        cryptocode
        environ
        floatrow
        forloop
        minted.tex
        patchedMintedBin
        pbox
        preprint
        sectsty
        titling
        upquote
        xargs
      ]
    ))
  ];

  languages.rust.enable = true;

  # Nix's compiler wrapper adds hardening flags to every compile, among them
  # libc++ bounds checks, stack protectors, -fwrapv and zeroing registers on
  # return. The C++ benchmarks should measure the standard library as an
  # ordinary build gets it, so turn them all off.
  env.NIX_HARDENING_ENABLE = "";
}
