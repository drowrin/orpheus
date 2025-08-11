function BlockQuote(elem)
    local first_block = elem.content:at(1, nil)
    if first_block == nil or first_block.t ~= "Para" then
        return elem
    end

    local first = first_block.content:at(1, nil)
    if first == nil or first.text == nil or first.text ~= "!" then
        return elem
    end

    first_block.content:remove(1)
    elem.content:remove(1)

    return pandoc.List(
        {
            pandoc.RawInline("html5", "<details><summary>"),
        }
        .. first_block.content
        .. {
            pandoc.RawInline("html5", "</summary>"),
        }
        .. elem.content
        .. {
            pandoc.RawInline("html5", "</details>")
        }
    )
end
