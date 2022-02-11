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
      "lfrom_epoch_index" => 12,
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
              "value" => "prop2",
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
                "value" => "prop2",
                "id" => 7777,
              }
            ]
          }


      ],
      "withdrawn_routes" => ["example.some-prefix-test03", "example.some-prefix-test04", "example.some-prefix-test05"],

    }))

  end

end
