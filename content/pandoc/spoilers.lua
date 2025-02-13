function Para(elem)
    attrs = {}
    attrs["class"] = 'spoiler'
    attrs["hx-on:click"] = "this.classList.toggle('revealed')"

    newinlines = pandoc.List()
    collection = pandoc.List()

    for _, c in ipairs(elem.content) do
        if c.text ~= nil then
            s, e = string.find(c.text, "||")
            if s ~= nil then
                before = string.sub(c.text, 1, s - 1)
                after = string.sub(c.text, e + 1)
                if next(collection) == nil then
                    if before ~= "" then
                        table.insert(newinlines, pandoc.Str(before))
                    end
                    s, e = string.find(after, "||")
                    if s ~= nil then
                        table.insert(newinlines, pandoc.Span(string.sub(after, 1, s - 1), attrs))
                    else
                        table.insert(collection, pandoc.Str(after))
                    end
                else
                    if before ~= "" then
                        table.insert(collection, pandoc.Str(before))
                    end
                    table.insert(newinlines, pandoc.Span(collection, attrs))
                    if after ~= "" then
                        table.insert(newinlines, pandoc.Str(after))
                    end
                    collection = pandoc.List()
                end
            else
                if next(collection) == nil then
                    table.insert(newinlines, c)
                else
                    table.insert(collection, c)
                end
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
