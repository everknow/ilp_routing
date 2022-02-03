defmodule IlpRouting do

  def decode(_), do: :erlang.nif_error(:nif_not_loaded)
  def encode(_), do: :erlang.nif_error(:nif_not_loaded)

end
