# copied expressions from https://nixos.wiki/wiki/Rust
# and Mozilla's nix overlay README
# https://www.scala-native.org/en/latest/user/setup.html
let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-23.11";
  pkgs = import nixpkgs {
    config = { };
    overlays = [ ];
  };
in

pkgs.mkShell {
  packages = with pkgs; [
    (opencv.override {
      enableGtk3 = true;
     })
    llvmPackages.libclang.lib
  ];
  nativeBuildInputs = [ pkgs.pkg-config ];

  shellHook = ''
    export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib";
  '';
}
