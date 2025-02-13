function Header(elem)
    attr = {
        class = "header-link"
    }
    permalink = pandoc.Link("#", "#" .. elem.attr.identifier, nil, attr)
    toplink = pandoc.Link("↑", "#title", nil, attr)
    elem.content = elem.content .. {permalink, toplink}
    return elem
end
