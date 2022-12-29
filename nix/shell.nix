{ self, ... }: {
  perSystem =
    { config
    , self'
    , inputs'
    , pkgs
    , ...
    }:
    let
      formatters = [
        pkgs.rustfmt
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
            pkgs.cargo-watch
          ]
          ++ self'.packages.lightning-gui.buildInputs;
        RUST_BACKTRACE = 1;
        nativeBuildInputs = [ ] ++ self'.packages.lightning-gui.nativeBuildInputs;
        passthru = {
          inherit formatters;
        };
      };
    };
}
