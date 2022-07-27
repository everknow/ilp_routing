defmodule IlpRoutingTest do
  use ExUnit.Case
  doctest IlpRouting

  test "encode/1 control request" do

    assert {:error, "type missing"} = IlpRouting.encode(%{})

    assert {:error, "could not decode arg to map<String,Term>"} = IlpRouting.encode(%{type: :some})

    assert {:error, "type not binary"} = IlpRouting.encode(%{"type" => :some})

    assert {:error, "type not recognised"} = IlpRouting.encode(%{"type" => "hello"})

    assert {:error, "control_request > features missing"} = IlpRouting.encode(%{"type" => "control_request"})

    assert {:error, "could not decode arg to map<String,Term>"} = IlpRouting.encode(%{"type" => "control_request", :features => :some})

    assert {:error, "could not decode last_known_routing_table_id"} = IlpRouting.encode(%{
      "type" => "control_request",
      "features" => :some,
      "last_known_epoch" => :some,
      "last_known_routing_table_id" => :some,
      "mode" => :some
    })

    assert {:error, "could not convert last_known_routing_table_id to list of bytes of size ROUTING_TABLE_ID_LEN"} = IlpRouting.encode(%{
      "type" => "control_request",
      "features" => ["hello", "there"],
      "last_known_epoch" => 345345,
      "last_known_routing_table_id" => [0],
      "mode" => 0
    })

    assert {:error, "last_known_epoch not u32"} = IlpRouting.encode(%{
      "type" => "control_request",
      "features" => ["Test", "one"],
      "last_known_epoch" => 79.33,
      "last_known_routing_table_id" => [0],
      "mode" => 0
    })

    assert is_list(IlpRouting.encode(%{
      "type" => "control_request",
      "features" => ["aa","bb"],
      "last_known_epoch" => 32,
      "last_known_routing_table_id" => [0,1,2,3,4,5,6,7,8,9,1,2,3,4,5,6],
      "mode" => 0
    }))

  end

  test "encode/1 update request" do

    assert is_list(IlpRouting.encode(%{
      "type" => "update_request",
      "routing_table_id" => [0,1,2,3,4,5,6,7,8,9,1,2,3,4,5,6],
      "current_epoch_index" => 30,
      "from_epoch_index" => 12,
      "to_epoch_index" => 20,
      "hold_down_time" => 3000,
      "speaker" => "example.aa",
      "new_routes" => [
        %{
          "prefix"=> "example.Prefix-Example-01",
          "path"=> ["example.some-prefix-test01", "example.some-prefix-test02", "example.some-prefix-test03"],
          "auth"=> [0,1,2,3,4,5,6,7,8,9,1,2,3,4,5,6,0,1,2,3,4,5,6,7,8,9,1,2,3,4,5,6],
          "props" => [
            %{
              "is_optional"=> false,
              "is_transitive"=> true,
              "is_partial"=> true,
              "id"=> 1111,
              "is_utf8"=> true,
              "value" => "test",
            },
            %{
              "is_optional" => false,
              "is_partial" => false,
              "is_utf8" => false,
              "is_transitive" => false,
              "value" => "\xa0\xa0",
              "id" => 7777,
            }
          ]
        },

        %{

          "prefix"=> "example.Prefix-Example-02",
          "path"=> ["example.some-prefix-test03", "example.some-prefix-test04", "example.some-prefix-test05"],
          "auth"=> [0,1,2,3,4,5,6,7,8,9,1,2,3,4,5,6,0,1,2,3,4,5,6,7,8,9,1,2,3,4,5,6],
          "props" => [
              %{
                "is_optional"=> false,
                "is_transitive"=> true,
                "is_partial"=> true,
                "id"=> 1111,
                "is_utf8"=> true,
                "value" => "test test test",
              },
              %{
                "is_optional" => false,
                "is_partial" => false,
                "is_utf8" => false,
                "is_transitive" => false,
                "value" => "\xa0\xa0",
                "id" => 7777,
              }
            ]
          }


      ],
      "withdrawn_routes" => ["example.some-prefix-test03", "example.some-prefix-test04", "example.some-prefix-test05"],

      }))

  end

  test "decode_control/1" do 
    binary = <<0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 0, 0, 0, 32, 1, 2, 2, 97,
      97, 2, 98, 98>>    
    control_request = IlpRouting.decode_control(binary)

    assert %{
      "type" => "control_request",
      "features" => ["aa","bb"],
      "last_known_epoch" => 32,
      "last_known_routing_table_id" => [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6],
      "mode" => 0
    } == control_request

  end

  test "decode_update/1" do 
        binary = <<0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 0, 0, 0, 30, 0, 0, 0, 12, 0, 0,
 0, 20, 0, 0, 11, 184, 10, 101, 120, 97, 109, 112, 108, 101, 46, 97, 97, 1, 2,
 25, 101, 120, 97, 109, 112, 108, 101, 46, 80, 114, 101, 102, 105, 120, 45, 69,
 120, 97, 109, 112, 108, 101, 45, 48, 49, 1, 3, 26, 101, 120, 97, 109, 112, 108,
 101, 46, 115, 111, 109, 101, 45, 112, 114, 101, 102, 105, 120, 45, 116, 101,
 115, 116, 48, 49, 26, 101, 120, 97, 109, 112, 108, 101, 46, 115, 111, 109, 101,
 45, 112, 114, 101, 102, 105, 120, 45, 116, 101, 115, 116, 48, 50, 26, 101, 120,
 97, 109, 112, 108, 101, 46, 115, 111, 109, 101, 45, 112, 114, 101, 102, 105,
 120, 45, 116, 101, 115, 116, 48, 51, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4,
 5, 6, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 1, 2, 112, 4, 87, 4, 116,
 101, 115, 116, 0, 30, 97, 5, 112, 114, 111, 112, 50, 25, 101, 120, 97, 109,
 112, 108, 101, 46, 80, 114, 101, 102, 105, 120, 45, 69, 120, 97, 109, 112, 108,
 101, 45, 48, 50, 1, 3, 26, 101, 120, 97, 109, 112, 108, 101, 46, 115, 111, 109,
 101, 45, 112, 114, 101, 102, 105, 120, 45, 116, 101, 115, 116, 48, 51, 26, 101,
 120, 97, 109, 112, 108, 101, 46, 115, 111, 109, 101, 45, 112, 114, 101, 102,
 105, 120, 45, 116, 101, 115, 116, 48, 52, 26, 101, 120, 97, 109, 112, 108, 101,
 46, 115, 111, 109, 101, 45, 112, 114, 101, 102, 105, 120, 45, 116, 101, 115,
 116, 48, 53, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5,
 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 1, 2, 112, 4, 87, 14, 116, 101, 115, 116, 32,
 116, 101, 115, 116, 32, 116, 101, 115, 116, 0, 30, 97, 2, 160, 160, 1, 3, 26,
 101, 120, 97, 109, 112, 108, 101, 46, 115, 111, 109, 101, 45, 112, 114, 101,
 102, 105, 120, 45, 116, 101, 115, 116, 48, 51, 26, 101, 120, 97, 109, 112, 108,
 101, 46, 115, 111, 109, 101, 45, 112, 114, 101, 102, 105, 120, 45, 116, 101,
 115, 116, 48, 52, 26, 101, 120, 97, 109, 112, 108, 101, 46, 115, 111, 109, 101,
 45, 112, 114, 101, 102, 105, 120, 45, 116, 101, 115, 116, 48, 53>>
    control_request = IlpRouting.decode_update(binary)

    assert %{
      "type" => "update_request",
      "routing_table_id" => [0,1,2,3,4,5,6,7,8,9,1,2,3,4,5,6],
      "current_epoch_index" => 30,
      "from_epoch_index" => 12,
      "to_epoch_index" => 20,
      "hold_down_time" => 3000,
      "speaker" => "example.aa",
      "new_routes" => [
        %{
          "prefix"=> "example.Prefix-Example-01",
          "path"=> ["example.some-prefix-test01", "example.some-prefix-test02", "example.some-prefix-test03"],
          "auth"=> [0,1,2,3,4,5,6,7,8,9,1,2,3,4,5,6,0,1,2,3,4,5,6,7,8,9,1,2,3,4,5,6],
          "props" => [
            %{
              "is_optional"=> false,
              "is_transitive"=> true,
              "is_partial"=> true,
              "id"=> 1111,
              "is_utf8"=> true,
              "value" => 'test',
            },
            %{
              "is_optional" => false,
              "is_partial" => false,
              "is_utf8" => false,
              "is_transitive" => false,
              "value" => 'prop2',
              "id" => 7777,

            }
          ]
        },

        %{

          "prefix"=> "example.Prefix-Example-02",
          "path"=> ["example.some-prefix-test03", "example.some-prefix-test04", "example.some-prefix-test05"],
          "auth"=> [0,1,2,3,4,5,6,7,8,9,1,2,3,4,5,6,0,1,2,3,4,5,6,7,8,9,1,2,3,4,5,6],
          "props" => [
              %{
                "is_optional"=> false,
                "is_transitive"=> true,
                "is_partial"=> true,
                "id"=> 1111,
                "is_utf8"=> true,
                "value" => 'test test test',
              },
              %{
                "is_optional" => false,
                "is_partial" => false,
                "is_utf8" => false,
                "is_transitive" => false,
                "value" => [160, 160],
                "id" => 7777,
              }
            ]
          }


      ],
      "withdrawn_routes" => ["example.some-prefix-test03", "example.some-prefix-test04", "example.some-prefix-test05"],

      } == control_request

  end

end
