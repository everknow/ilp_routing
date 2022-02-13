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

    assert is_list(IlpRouting.encode(IlpRoutingHelper.update_request))

  end

  test "encode decode" do
    assert :update == IlpRoutingHelper.update_request |> IlpRouting.encode |> :binary.list_to_bin() |> IlpRouting.decode
  end

end
