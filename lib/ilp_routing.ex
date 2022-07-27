defmodule IlpRouting do

  use Rustler, otp_app: :ilp_routing, crate: "routing"

  def decode_control(_), do: :erlang.nif_error(:nif_not_loaded)
  def decode_update(_), do: :erlang.nif_error(:nif_not_loaded)
  def encode(_), do: :erlang.nif_error(:nif_not_loaded)

end
