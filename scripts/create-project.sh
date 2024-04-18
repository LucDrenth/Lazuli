#!/bin/bash

########################################################
# We start with some constant and function definitions #
########################################################

COLOR_RED='\033[1;31m'
COLOR_CYAN='\033[1;36m'
COLOR_RESET='\033[0m'

# $1 = color
# $2 = message
function log() {
    printf "\n${1}${2} ${COLOR_RESET}\n\n"
}

function log_info() {
    log "${COLOR_CYAN}" "$1"
}
function log_err() {
    log "${COLOR_RED}" "$1"
}

NEW_PROJECT_NAME=$1
LAZULI_DIRECTORY=$(realpath "$(dirname "$0")/..")
TARGET_DIRECTORY=$(realpath "$(dirname "$LAZULI_DIRECTORY")")/$NEW_PROJECT_NAME



#################################
# The actual script starts here #
#################################


# Validate the given project name
if [ -z "$NEW_PROJECT_NAME" ]; then
    log_err "Please specify a project name"
    exit 1;
fi

if [ -d "$TARGET_DIRECTORY" ]; then
    log_err "Project directory for '$NEW_PROJECT_NAME' already exists: $TARGET_DIRECTORY"
    exit 1;
fi

# Step in to the directory where we will create the project
cd $LAZULI_DIRECTORY/..

# Create our new project
log_info "Creating new Lazuli project: $NEW_PROJECT_NAME"
cargo new $NEW_PROJECT_NAME

# Step in to the newly created project
cd ./$NEW_PROJECT_NAME

# Add Lazuli as a dependency
log_info "Adding Lazuli as a dependency"
cargo add lazuli --path ../lazuli

# Copy over assets from Lazuli to our new project
log_info "Copying over Lazuli assets"

printf "engine assets\n"
cp -r ${LAZULI_DIRECTORY}/assets ./

printf "vscode settings\n"
cp -r ${LAZULI_DIRECTORY}/.vscode ./

printf "run script\n"
mkdir scripts && cp ${LAZULI_DIRECTORY}/scripts/run.sh ./scripts/

printf ".gitignore\n"
cp ${LAZULI_DIRECTORY}/project-template/gitignore-template ./.gitignore

printf "src folder\n"
cp -r ${LAZULI_DIRECTORY}/project-template/src ./

printf "README.md\n"
cp ${LAZULI_DIRECTORY}/project-template/README.md ./

# Getting here means we successfully created the project
log_info "Successfully created project '${NEW_PROJECT_NAME}'"

# If we are running on macOS, open the newly created project in the finder
if [[ "$OSTYPE" == "darwin"* ]]; then
    open $TARGET_DIRECTORY
fi

exit 0
