#!/bin/bash -e

replace_placeholders() {
    local folder="$1"
    local proton_name="$2"
    local proton_build_id="$3"
    echo "Replacing placeholders with ${proton_name} (${proton_build_id}) in ${folder}"
    sed -i "s|\${PROTON_NAME}|${proton_name}|g" "$folder/launch.sh"
    sed -i "s|\${PROTON_BUILD_ID}|${proton_build_id}|g" "$folder/launch.sh"
    sed -i "s|\${PROTON_NAME}|${proton_name}|g" "$folder/compatibilitytool.vdf"
    sed -i "s|\${PROTON_BUILD_ID}|${proton_build_id}|g" "$folder/compatibilitytool.vdf"
}

if [ $# -ne 2 ]; then
    echo "Usage: $0 <build_name> <output_dir>"
    exit 1
fi

PROTON_BUILD_NAME="$1"
OUTPUT_FOLDER="$2"
BASE_PATH="$(pwd)/steam-tool"
case "$PROTON_BUILD_NAME" in
    "tkg-wine")
        PROTON_NAME="Proton-Tkg-Wine-Fusion"
        PROTON_BUILD_ID="proton-tkg-wine"
        ;;
    "tkg-valvebe")
        PROTON_NAME="Proton-Tkg-Valvebe-Fusion"
        PROTON_BUILD_ID="proton-tkg-valvebe"
        ;;
    "ge")
        PROTON_NAME="GE-Proton-Fusion"
        PROTON_BUILD_ID="proton-ge"
        ;;
    *)
        echo "Unknown Proton build name: ${PROTON_BUILD_NAME}. Valid options are: tkg-wine, tkg-valvebe, ge."
        exit 1
        ;;
esac

# Start build
echo "Creating build for ${PROTON_BUILD_NAME} at ${OUTPUT_FOLDER}"
mkdir -p "$OUTPUT_FOLDER"

# Build and copy the updater.
echo
echo "Building and copying proton updater"
cd proton-updater
cargo build --release
if [ $? -ne 0 ]; then
    echo "Failed to build updater, aborting"
    exit 1
fi
cd ..
cp ./proton-updater/target/release/proton-updater "$OUTPUT_FOLDER"

# Format and copy the compat tool files.
echo
echo "Copying and formatting compat tool files"
cp -r "$BASE_PATH/"* "$OUTPUT_FOLDER"
replace_placeholders "$OUTPUT_FOLDER" "$PROTON_NAME" "$PROTON_BUILD_ID"

# We done
echo
echo "Successfully built steam tool ${PROTON_BUILD_NAME}"
