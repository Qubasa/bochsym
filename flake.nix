{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    nix-fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, nix-fenix, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ nix-fenix.overlay ];
        pkgs = import nixpkgs { inherit system; inherit overlays; };
        naersk-lib = pkgs.callPackage naersk { };

        fenix = nix-fenix.packages.${system};
        target64 = fenix.targets."x86_64-unknown-none".latest.withComponents [
          "rust-std"
        ];
        myrust = with fenix; fenix.combine [
          (latest.withComponents [
            "rust-src"
            "rustc"
            "rustfmt"
            "llvm-tools-preview"
            "cargo"
            "clippy"
          ])
          target64
        ];

        buildDeps = with pkgs; [
          myrust
          zlib.out
          xorriso
          grub2
        ]  ++ (with pkgs.llvmPackages_latest; [
          lld
          llvm
        ]);
      in
      rec {
        packages.default = naersk-lib.buildPackage ./.;

        defaultPackage = packages.default;

        apps.default = utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
        };

        devShell = with pkgs; mkShell {
          buildInputs = buildDeps;
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          shellHook = ''
          export PATH=$PATH:~/.cargo/bin
          '';
        };
      });
}
