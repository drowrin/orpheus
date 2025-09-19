function BlockQuote(elem)
    for _, block in ipairs(elem.content) do
        if block.t == "Para" then
            local first = block.content:at(1, nil)
            if first ~= nil and first.text ~= nil and first.text == "—" then
                block.content:insert(
                    1,
                    pandoc.RawInline(
                        "html5",
                        "<footer><cite>"
                    )
                )
                block.content:insert(
                    pandoc.RawInline(
                        "html5",
                        "</cite></footer>"
                    )
                )
            end
        end
    end

    return elem
end
