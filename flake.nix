{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;

        config = {
          allowUnfree = true;
          android_sdk.accept_license = true;
        };
      };

      # TODO copy these from the SDL3
      androidVersion = {
        buildTools = "35.0.0";
        sdk = "36";
        ndk = "28.2.13676358";
        cmake = "3.22.1";
      };

      androidComposition = pkgs.androidenv.composeAndroidPackages {
        buildToolsVersions = [ androidVersion.buildTools ];
        platformVersions = [ androidVersion.sdk ];
        ndkVersions = [ androidVersion.ndk ];
        includeNDK = true;
        cmakeVersions = [ androidVersion.cmake ];
      };

      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        targets = [
          "aarch64-linux-android"
          # "armv7-linux-androideabi"
          # "x86_64-linux-android"
        ];
      };
    in
    {
      devShells.${system} = rec {
        default = desktop;

        desktop = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [
            git

            cargo
            rust-analyzer
            clippy
            rustfmt
            rustc

            # basically SDL3 dependencies
            libxtst
            libxcb
            just
            cmake
            validatePkgConfig
            gcc
            wayland-scanner
            zenity
            libffi
            python313
            patchelf
            vulkan-headers
            vulkan-loader
            libGL
            libusb1
            libayatana-appindicator
            libdrm
            mesa
            wayland
            wayland-protocols
            pipewire
            libpulseaudio
            alsa-lib
            dbus
            libxkbcommon
            xorg.libX11
            xorg.libXScrnSaver
            xorg.libXcursor
            xorg.libXext
            xorg.libXfixes
            xorg.libXi
            xorg.libXrandr
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeBuildInputs;
        };

        # TODO add rust-overlay and aarch64-linux-android target
        android = pkgs.mkShell rec {
          inputsFrom = [ desktop ];

          nativeBuildInputs = with pkgs; [
            androidComposition.androidsdk
            androidComposition.platform-tools
            androidComposition.cmake
            javaPackages.compiler.openjdk17

            # rust
            rustToolchain
            cargo-ndk
          ];

          ANDROID_HOME = "${androidComposition.androidsdk}/libexec/android-sdk";
          ANDROID_SDK_ROOT = "${androidComposition.androidsdk}/libexec/android-sdk";
          ANDROID_NDK_ROOT = "${ANDROID_HOME}/ndk-bundle";
          GRADLE_OPTS = "-Dorg.gradle.project.android.aapt2FromMavenOverride=${ANDROID_HOME}/build-tools/${androidVersion.buildTools}/aapt2";
          CARGO_BUILD_TARGET = "aarch64-linux-android";
        };
      };
    };
}
