// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: shapes.proto

package vsg.shapes;

public interface CrossOrBuilder extends
    // @@protoc_insertion_point(interface_extends:vsg.Cross)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <code>float size = 1;</code>
   * @return The size.
   */
  float getSize();

  /**
   * <code>float line_width = 2;</code>
   * @return The lineWidth.
   */
  float getLineWidth();

  /**
   * <code>.vsg.Coordinates ctr = 3;</code>
   * @return Whether the ctr field is set.
   */
  boolean hasCtr();
  /**
   * <code>.vsg.Coordinates ctr = 3;</code>
   * @return The ctr.
   */
  vsg.shapes.Coordinates getCtr();
  /**
   * <code>.vsg.Coordinates ctr = 3;</code>
   */
  vsg.shapes.CoordinatesOrBuilder getCtrOrBuilder();
}