{ self, ... }:
{
  perSystem = { self', pkgs, ... }:
    {
      checks.format = pkgs.callPackage ./check-format.nix {
        inherit self;
        inherit (self'.devShells.default) formatters;
      };
      checks.test = self'.packages.lightning-gui.override {
        enableTests = true;
      };
      checks.lint = self'.packages.lightning-gui.override {
        enableLint = true;
      };
    };
}
