defmodule IlpRoutingHelper do

  def update_request, do: %{
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

  }


end
