# frozen_string_literal: true
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: proto/games.proto

require 'google/protobuf'

require 'google/protobuf/timestamp_pb'


descriptor_data = "\n\x11proto/games.proto\x12\x10proto.playground\x1a\x1fgoogle/protobuf/timestamp.proto\")\n\x16\x43\x61ncelLastScoreRequest\x12\x0f\n\x07game_id\x18\x01 \x01(\t\"?\n\x17\x43\x61ncelLastScoreResponse\x12$\n\x04game\x18\x01 \x01(\x0b\x32\x16.proto.playground.Game\"5\n\x12\x43ountPointsRequest\x12\x0f\n\x07game_id\x18\x01 \x01(\t\x12\x0e\n\x06points\x18\x02 \x01(\x05\";\n\x13\x43ountPointsResponse\x12$\n\x04game\x18\x01 \x01(\x0b\x32\x16.proto.playground.Game\"\x13\n\x11\x43reateGameRequest\"!\n\x0eGetGameRequest\x12\x0f\n\x07game_id\x18\x01 \x01(\t\"\x12\n\x10ListGamesRequest\":\n\x11ListGamesResponse\x12%\n\x05games\x18\x01 \x03(\x0b\x32\x16.proto.playground.Game\"\x94\x02\n\x04Game\x12\n\n\x02id\x18\x01 \x01(\t\x12\x0e\n\x06winner\x18\x02 \x01(\t\x12\x0e\n\x06player\x18\x03 \x01(\t\x12\x1c\n\x14player_points_to_win\x18\x04 \x01(\x05\x12/\n\x0b\x63reate_time\x18\x05 \x01(\x0b\x32\x1a.google.protobuf.Timestamp\x12/\n\x0bupdate_time\x18\x06 \x01(\x0b\x32\x1a.google.protobuf.Timestamp\x12\'\n\x06rounds\x18\x07 \x03(\x0b\x32\x17.proto.playground.Round\x12\x37\n\x0eplayer_details\x18\x08 \x03(\x0b\x32\x1f.proto.playground.PlayerDetails\"4\n\rPlayerDetails\x12\x15\n\rpoints_to_win\x18\x01 \x01(\x05\x12\x0c\n\x04name\x18\x02 \x01(\t\"@\n\x05Round\x12\x0e\n\x06number\x18\x01 \x01(\x05\x12\'\n\x06points\x18\x02 \x03(\x0b\x32\x17.proto.playground.Point\"A\n\x05Point\x12\r\n\x05value\x18\x01 \x01(\x05\x12)\n\x04kind\x18\x02 \x01(\x0e\x32\x1b.proto.playground.PointKind*S\n\tPointKind\x12\x14\n\x10POINT_KIND_UNSET\x10\x00\x12\x16\n\x12POINT_KIND_REGULAR\x10\x01\x12\x18\n\x14POINT_KIND_OVERTHROW\x10\x02\x32\xb1\x03\n\x05Games\x12\x66\n\x0f\x43\x61ncelLastScore\x12(.proto.playground.CancelLastScoreRequest\x1a).proto.playground.CancelLastScoreResponse\x12Z\n\x0b\x43ountPoints\x12$.proto.playground.CountPointsRequest\x1a%.proto.playground.CountPointsResponse\x12I\n\nCreateGame\x12#.proto.playground.CreateGameRequest\x1a\x16.proto.playground.Game\x12\x43\n\x07GetGame\x12 .proto.playground.GetGameRequest\x1a\x16.proto.playground.Game\x12T\n\tListGames\x12\".proto.playground.ListGamesRequest\x1a#.proto.playground.ListGamesResponseb\x06proto3"

pool = Google::Protobuf::DescriptorPool.generated_pool

begin
  pool.add_serialized_file(descriptor_data)
rescue TypeError
  # Compatibility code: will be removed in the next major version.
  require 'google/protobuf/descriptor_pb'
  parsed = Google::Protobuf::FileDescriptorProto.decode(descriptor_data)
  parsed.clear_dependency
  serialized = parsed.class.encode(parsed)
  file = pool.add_serialized_file(serialized)
  warn "Warning: Protobuf detected an import path issue while loading generated file #{__FILE__}"
  imports = [
    ["google.protobuf.Timestamp", "google/protobuf/timestamp.proto"],
  ]
  imports.each do |type_name, expected_filename|
    import_file = pool.lookup(type_name).file_descriptor
    if import_file.name != expected_filename
      warn "- #{file.name} imports #{expected_filename}, but that import was loaded as #{import_file.name}"
    end
  end
  warn "Each proto file must use a consistent fully-qualified name."
  warn "This will become an error in the next major version."
end

module Proto
  module Playground
    CancelLastScoreRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("proto.playground.CancelLastScoreRequest").msgclass
    CancelLastScoreResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("proto.playground.CancelLastScoreResponse").msgclass
    CountPointsRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("proto.playground.CountPointsRequest").msgclass
    CountPointsResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("proto.playground.CountPointsResponse").msgclass
    CreateGameRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("proto.playground.CreateGameRequest").msgclass
    GetGameRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("proto.playground.GetGameRequest").msgclass
    ListGamesRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("proto.playground.ListGamesRequest").msgclass
    ListGamesResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("proto.playground.ListGamesResponse").msgclass
    Game = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("proto.playground.Game").msgclass
    PlayerDetails = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("proto.playground.PlayerDetails").msgclass
    Round = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("proto.playground.Round").msgclass
    Point = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("proto.playground.Point").msgclass
    PointKind = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("proto.playground.PointKind").enummodule
  end
end
