{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, fenix, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        toolchain = with fenix.packages.${system}; combine [
          complete.toolchain
          targets.wasm32-unknown-unknown.latest.rust-std
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            toolchain
            pkgs.wasm-pack
            pkgs.rust-analyzer
            pkgs.deno
          ] ++ (with pkgs; [
            pkg-config
            openssl
            # Add LLVM tools for WASM linking
            llvmPackages.bintools
          ]);

          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
          # Set the linker for WASM
          CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
        };
      }
    );
}
