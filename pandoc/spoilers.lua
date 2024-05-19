function Para(elem)
    attrs = {}
    attrs["class"] = 'spoiler'
    attrs["hx-on:click"] = "this.classList.toggle('revealed')"

    newinlines = pandoc.List()
    collection = pandoc.List()

    for _, c in ipairs(elem.content) do
        if c.text ~= nil and string.find(c.text, "||") then
            if next(collection) == nil then
                table.insert(collection, pandoc.Str(string.sub(c.text, 3)))
            else
                table.insert(collection, pandoc.Str(string.sub(c.text, 1, -3)))
                table.insert(newinlines, pandoc.Span(collection, attrs))
                collection = pandoc.List()
            end
        else
            if next(collection) == nil then
                table.insert(newinlines, c)
            else
                table.insert(collection, c)
            end
        end
    end

    if next(collection) ~= nil then
        newinlines = newinlines .. collection
    end

    elem.content = newinlines
    return elem
end
