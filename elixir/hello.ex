parent = self()

spawn_link(fn ->
  send parent, {:msg, "hello world"}
end)

receive do
  {:msg, contents} -> IO.puts contents
end
