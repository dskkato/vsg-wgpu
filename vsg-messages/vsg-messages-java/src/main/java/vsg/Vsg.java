// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: message.proto

package vsg;

public final class Vsg {
  private Vsg() {}
  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistryLite registry) {
  }

  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistry registry) {
    registerAllExtensions(
        (com.google.protobuf.ExtensionRegistryLite) registry);
  }
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_vsg_RootMessage_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_vsg_RootMessage_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_vsg_BgColor_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_vsg_BgColor_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_vsg_Texture_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_vsg_Texture_fieldAccessorTable;

  public static com.google.protobuf.Descriptors.FileDescriptor
      getDescriptor() {
    return descriptor;
  }
  private static  com.google.protobuf.Descriptors.FileDescriptor
      descriptor;
  static {
    java.lang.String[] descriptorData = {
      "\n\rmessage.proto\022\003vsg\032\014shapes.proto\"\204\001\n\013R" +
      "ootMessage\022\037\n\tset_shape\030\001 \001(\0132\n.vsg.Shap" +
      "eH\000\022$\n\014set_bg_color\030\002 \001(\0132\014.vsg.BgColorH" +
      "\000\022#\n\013set_texture\030\003 \001(\0132\014.vsg.TextureH\000B\t" +
      "\n\007command\"\030\n\007BgColor\022\r\n\005color\030\001 \003(\002\"&\n\007T" +
      "exture\022\r\n\005index\030\001 \001(\r\022\014\n\004data\030\002 \001(\014B\014\n\003v" +
      "sgB\003VsgP\001b\006proto3"
    };
    descriptor = com.google.protobuf.Descriptors.FileDescriptor
      .internalBuildGeneratedFileFrom(descriptorData,
        new com.google.protobuf.Descriptors.FileDescriptor[] {
          vsg.shapes.VsgShapes.getDescriptor(),
        });
    internal_static_vsg_RootMessage_descriptor =
      getDescriptor().getMessageTypes().get(0);
    internal_static_vsg_RootMessage_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_vsg_RootMessage_descriptor,
        new java.lang.String[] { "SetShape", "SetBgColor", "SetTexture", "Command", });
    internal_static_vsg_BgColor_descriptor =
      getDescriptor().getMessageTypes().get(1);
    internal_static_vsg_BgColor_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_vsg_BgColor_descriptor,
        new java.lang.String[] { "Color", });
    internal_static_vsg_Texture_descriptor =
      getDescriptor().getMessageTypes().get(2);
    internal_static_vsg_Texture_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_vsg_Texture_descriptor,
        new java.lang.String[] { "Index", "Data", });
    vsg.shapes.VsgShapes.getDescriptor();
  }

  // @@protoc_insertion_point(outer_class_scope)
}