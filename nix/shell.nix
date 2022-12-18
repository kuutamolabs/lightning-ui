{ self, ... }: {
  perSystem =
    { config
    , self'
    , inputs'
    , pkgs
    , ...
    }:
    let
      rust = (pkgs.rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
              targets = [ "wasm32-unknown-unknown" ];
            });
      formatters = [
        pkgs.treefmt
        pkgs.nixpkgs-fmt
      ];
    in
    {
      devShells.default = pkgs.mkShell {
        buildInputs =
          formatters ++ [
            # tasks and automation
            pkgs.just
            pkgs.nix-update

            # rust dev
            rust
            pkgs.cargo-watch
            pkgs.trunk
          ];
        RUST_BACKTRACE = 1;
        nativeBuildInputs = [ rust ];
        passthru = {
          inherit formatters;
        };
      };
    };
}
