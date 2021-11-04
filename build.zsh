export APP_NAME=bevy-tutorial
export script_dir=$(cd $(dirname ${BASH_SOURCE:-$0}); pwd)
export TARGET=$script_dir/target/release/$APP_NAME

python3 $script_dir/build.py > $script_dir/build/info.plist

export CONTENTS_DIR=$script_dir/build/$APP_NAME.app/Contents
rm -rf $CONTENTS_DIR # clear
mkdir -p $CONTENTS_DIR
cd $CONTENTS_DIR
mkdir MacOS Resources
echo $script_dir
cp $TARGET MacOS
cp -r $script_dir/assets MacOS
cp $script_dir/build/info.plist .