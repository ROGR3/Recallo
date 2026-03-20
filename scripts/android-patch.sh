#!/usr/bin/env bash
# android-patch.sh — Post-dx-build fixups for the Android project
# Patches: icons, build.gradle SDK versions, 16KB alignment
# Usage: ./scripts/android-patch.sh <debug|release>
set -euo pipefail

MODE="${1:-debug}"
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ANDROID_APP="$PROJECT_ROOT/target/dx/recallo/$MODE/android/app/app"
LOGO="$PROJECT_ROOT/Logo.png"
RES="$ANDROID_APP/src/main/res"

echo "==> Patching Android project ($MODE)..."

# ── 1. Patch build.gradle.kts: bump compileSdk and targetSdk ──────────────
GRADLE="$ANDROID_APP/build.gradle.kts"
if [ -f "$GRADLE" ]; then
    sed -i 's/compileSdk = 33/compileSdk = 35/' "$GRADLE"
    sed -i 's/targetSdk = 33/targetSdk = 35/' "$GRADLE"
    echo "    [OK] build.gradle.kts: compileSdk=35, targetSdk=35"
else
    echo "    [WARN] build.gradle.kts not found at $GRADLE"
fi

# ── 2. Replace launcher icons with our logo ──────────────────────────────
if command -v magick &>/dev/null && [ -f "$LOGO" ]; then
    # Standard launcher icons
    magick "$LOGO" -resize 48x48   -quality 90 "$RES/mipmap-mdpi/ic_launcher.webp"
    magick "$LOGO" -resize 72x72   -quality 90 "$RES/mipmap-hdpi/ic_launcher.webp"
    magick "$LOGO" -resize 96x96   -quality 90 "$RES/mipmap-xhdpi/ic_launcher.webp"
    magick "$LOGO" -resize 144x144 -quality 90 "$RES/mipmap-xxhdpi/ic_launcher.webp"
    magick "$LOGO" -resize 192x192 -quality 90 "$RES/mipmap-xxxhdpi/ic_launcher.webp"

    # Remove the adaptive icon XML so Android uses the mipmap directly
    rm -f "$RES/mipmap-anydpi-v26/ic_launcher.xml"
    rm -f "$RES/drawable/ic_launcher_background.xml"
    rm -f "$RES/drawable-v24/ic_launcher_foreground.xml"

    echo "    [OK] Launcher icons replaced with Logo.png"
else
    echo "    [WARN] Skipping icon patch (need 'magick' and Logo.png)"
fi

# ── 3. Re-run gradle to rebuild APK with patches ─────────────────────────
echo "==> Re-running Gradle assembleDebug..."
cd "$ANDROID_APP/.."
./gradlew assembleDebug --quiet 2>&1 | tail -5
echo "==> Patch complete."
