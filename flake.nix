{
    description = "Rust Flake";
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    };
    outputs = {self, nixpkgs, ...}:
    let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    in
    {
        packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
            pname = "xxx";
            version = "0.1.0";

            src = pkgs.lib.cleanSource ./.;
            cargoLock.lockFile = ./Cargo.lock;
        };

        devShells.${system}.default = pkgs.mkShell {
            packages = with pkgs; [
                rustc
                cargo
                rustfmt
                clippy
                rust-analyzer
            ];
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };
    };
}
