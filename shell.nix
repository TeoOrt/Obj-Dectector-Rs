# copied expressions from https://nixos.wiki/wiki/Rust
# and Mozilla's nix overlay README
# https://www.scala-native.org/en/latest/user/setup.html
let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-25.05-small";
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
    (python312.withPackages ( python-pkgs: with python-pkgs; [
      gitpython
      matplotlib
      numpy
      pillow
      psutil
      pyyaml 
      requests
      scipy
      ultralytics-thop
      torch
      torchvision
      tqdm
      ultralytics
      pandas
      seaborn
      setuptools
      venvShellHook
      pip
      onnx
      opencv
    ]))
  ];
  nativeBuildInputs = [ pkgs.pkg-config ];

  shellHook = ''
    export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib";
  '';
}
