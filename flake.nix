{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      packages.${system} = rec {
        default = optarith;
        optarith = pkgs.callPackage ./optarith.nix { };
      };

      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          autoconf
          automake
          libtool
          pkg-config
          rustPlatform.bindgenHook
        ];

        buildInputs = with pkgs; [
          rustc
          cargo
          clippy
          rustfmt
          llvmPackages_latest.libclang
          llvmPackages_latest.clang
          clang-tools
        ];
      };
    };
}
