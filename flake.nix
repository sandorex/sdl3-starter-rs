{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
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

      # buildGradleContent = builtins.readFile "android-project/app/build.gradle";
      #
      # androidComposition = pkgs.androidenv.composeAndroidPackages {
      #   platformVersions = [
      #     "35"
      #   ];
      #   buildToolsVersions = [
      #     builtins.match ".*buildToolsVersion \"([^\"]*)\".*" buildGradleContent
      #     # "34.0.0"
      #   ];
      #   includeNDK = true;
      #   ndkVersions = [
      #     builtins.match ".*ndkVersion \"([^\"]*)\".*" buildGradleContent
      #     # "28.1.13356709"
      #   ];
      #   cmakeVersions = [
      #     # NOTE this is a fragile regex
      #     builtins.match ".*version \"([^\"]*)\".*" buildGradleContent
      #     # "3.22.1"
      #   ];
      #   abiVersions = [
      #     "armeabi-v7a"
      #     "arm64-v8a"
      #   ];
      # };

      # TODO automate extraction using regex?
      # TODO read JSON from gradle and read it here as well
      # copied from 'android-project/app/build.gradle'
      androidVersion = {
        buildTools = "34.0.0";
        sdk = "35";
        ndk = "28.2.13676358";
      };

      # buildToolsVersion = builtins.match ".*buildToolsVersion \"([^\"]*)\".*" buildGradleContent;
      androidComposition = pkgs.androidenv.composeAndroidPackages {
        buildToolsVersions = [ androidVersion.buildTools ];
        platformVersions = [ androidVersion.sdk ];
        ndkVersions = [ androidVersion.ndk ];
        includeNDK = true;
      };

      rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;

      sdlPkgs = with pkgs; [
        pkg-config
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
        libxtst
        libxcb
        libxkbcommon
        libx11
        libxscrnsaver
        libxcursor
        libxext
        libxfixes
        libxi
        libxrandr
      ];

      envVars = rec {
          # make SDL3 work properly, without this it cannot find video device
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath sdlPkgs;

          # point android to proper place
          ANDROID_HOME = "${androidComposition.androidsdk}/libexec/android-sdk";
          ANDROID_SDK_ROOT = "${androidComposition.androidsdk}/libexec/android-sdk";
          ANDROID_NDK_ROOT = "${ANDROID_HOME}/ndk-bundle";

          # force use of nix aapt2
          GRADLE_OPTS = "-Dorg.gradle.project.android.aapt2FromMavenOverride=${ANDROID_HOME}/build-tools/${androidVersion.buildTools}/aapt2";
      };
    in
    {
      devShells.${system} = {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            cargo-ndk

            # android
            androidComposition.androidsdk
            androidComposition.platform-tools
            javaPackages.compiler.openjdk17

            # emulator
            (androidenv.emulateApp {
              name = "emulate-android";
              platformVersion = "28";
              abiVersion = "x86_64";
              systemImageType = "google_apis_playstore";
            })
          ] ++ sdlPkgs;

          inherit (envVars) LD_LIBRARY_PATH ANDROID_HOME ANDROID_SDK_ROOT ANDROID_NDK_ROOT GRADLE_OPTS;
        };

        # desktop only shell (a lot smaller)
        desktop = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            cargo-ndk
          ] ++ sdlPkgs;

          inherit (envVars) LD_LIBRARY_PATH;
        };
      };
    };
}
