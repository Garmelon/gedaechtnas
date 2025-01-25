set -x JAVA_HOME /opt/android-studio/jbr
set -x ANDROID_HOME "$HOME/Android/Sdk"
set -x NDK_HOME "$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"
