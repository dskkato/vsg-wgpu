rm -r -fo vsg-messages-java/src/main/java/*
protoc --java_out=vsg-messages-java/src/main/java -Iproto proto/message.proto proto/shapes.proto