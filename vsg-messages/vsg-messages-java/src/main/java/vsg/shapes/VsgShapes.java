// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: shapes.proto

package vsg.shapes;

public final class VsgShapes {
  private VsgShapes() {}
  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistryLite registry) {
  }

  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistry registry) {
    registerAllExtensions(
        (com.google.protobuf.ExtensionRegistryLite) registry);
  }
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_vsg_Shape_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_vsg_Shape_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_vsg_Coordinates_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_vsg_Coordinates_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_vsg_Square_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_vsg_Square_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_vsg_Circle_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_vsg_Circle_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_vsg_Cross_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_vsg_Cross_fieldAccessorTable;

  public static com.google.protobuf.Descriptors.FileDescriptor
      getDescriptor() {
    return descriptor;
  }
  private static  com.google.protobuf.Descriptors.FileDescriptor
      descriptor;
  static {
    java.lang.String[] descriptorData = {
      "\n\014shapes.proto\022\003vsg\"k\n\005Shape\022\035\n\006square\030\001" +
      " \001(\0132\013.vsg.SquareH\000\022\035\n\006circle\030\002 \001(\0132\013.vs" +
      "g.CircleH\000\022\033\n\005cross\030\003 \001(\0132\n.vsg.CrossH\000B" +
      "\007\n\005shape\"#\n\013Coordinates\022\t\n\001x\030\001 \001(\002\022\t\n\001y\030" +
      "\002 \001(\002\"5\n\006Square\022\014\n\004size\030\001 \001(\002\022\035\n\003ctr\030\002 \001" +
      "(\0132\020.vsg.Coordinates\"7\n\006Circle\022\016\n\006radius" +
      "\030\001 \001(\002\022\035\n\003ctr\030\002 \001(\0132\020.vsg.Coordinates\"H\n" +
      "\005Cross\022\014\n\004size\030\001 \001(\002\022\022\n\nline_width\030\002 \001(\002" +
      "\022\035\n\003ctr\030\003 \001(\0132\020.vsg.CoordinatesB\031\n\nvsg.s" +
      "hapesB\tVsgShapesP\001b\006proto3"
    };
    descriptor = com.google.protobuf.Descriptors.FileDescriptor
      .internalBuildGeneratedFileFrom(descriptorData,
        new com.google.protobuf.Descriptors.FileDescriptor[] {
        });
    internal_static_vsg_Shape_descriptor =
      getDescriptor().getMessageTypes().get(0);
    internal_static_vsg_Shape_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_vsg_Shape_descriptor,
        new java.lang.String[] { "Square", "Circle", "Cross", "Shape", });
    internal_static_vsg_Coordinates_descriptor =
      getDescriptor().getMessageTypes().get(1);
    internal_static_vsg_Coordinates_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_vsg_Coordinates_descriptor,
        new java.lang.String[] { "X", "Y", });
    internal_static_vsg_Square_descriptor =
      getDescriptor().getMessageTypes().get(2);
    internal_static_vsg_Square_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_vsg_Square_descriptor,
        new java.lang.String[] { "Size", "Ctr", });
    internal_static_vsg_Circle_descriptor =
      getDescriptor().getMessageTypes().get(3);
    internal_static_vsg_Circle_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_vsg_Circle_descriptor,
        new java.lang.String[] { "Radius", "Ctr", });
    internal_static_vsg_Cross_descriptor =
      getDescriptor().getMessageTypes().get(4);
    internal_static_vsg_Cross_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_vsg_Cross_descriptor,
        new java.lang.String[] { "Size", "LineWidth", "Ctr", });
  }

  // @@protoc_insertion_point(outer_class_scope)
}