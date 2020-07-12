'use strict';

// API
function GetIndex(setIndex, by, searched) {
    fetch('/neu/index.json')
        .then(response => response.json())
        .then(data => {
            let result;
            searched = searched ?? "";
            searched = searched.toLowerCase();
            let filter = value =>
                value.title.toLowerCase().includes(searched) ||
                value.kind.toLowerCase().includes(searched) ||
                value.id.toLowerCase().includes(searched)
            ;
            if(!by) {
                result = data.data.filter(filter);
            } else {
                const idx = data[by];
                result = [];
                for (const entry in idx) {
                    const entry_idx = idx[entry].map(i => data.data[i]);
                    result.push([entry, entry_idx]);
                }
                result = result.map(group => {
                    group[1] = group[1].filter(filter);
                    return group;
                })
                .filter(group => group[1].length > 0);
            }


            setIndex(result);
        });
}

// GUI

const { useState, useEffect, createElement } = React;
const e = createElement;

function Article({kind, id}) {
    const [html, setHtml] = useState("Loading...");
    const [hasTitle, setTitle] = useState(false);

    useEffect(() => {
        fetch(`/neu/articles/${kind}/${id}.html`)
            .then(response => {
                if (!response.ok) {
                    if(response.status === 404) {
                        throw 'Not found';
                    }
                    throw new Error('Network response was not ok');
                }
                return response.text();
            })
            .then(data => { setHtml(data); })
            .catch(err => {
                setHtml(`Couldn't load article: ${err}`);
            });
    }, [ kind, id ]);

    useEffect(() => {
        const div_id = `${kind}_${id}`;
        const div = document.getElementById(div_id);
        if(div && div.classList.contains('article-item')) {
            div.scrollIntoView();
        }
    });

    useEffect(() => {
        GetIndex(data => {
            let meta = data.find(val => val.kind === kind && val.id === id);
            if(meta) {
                document.title = `${meta.title}`;
                if(hasTitle) {
                    window.history.pushState("", meta.title, `/${kind}/${id}`);
                } else {
                    window.history.replaceState("", meta.title, `/${kind}/${id}`);
                    setTitle(true);
                }
            }
        });
        // TODO: Replace with article metadata? Or at least cache http
    }, [ kind, id ]);

    return e('div', { className: 'article' }, [
        e('div', { key: "html", dangerouslySetInnerHTML: { __html: html } })
    ]);
}

function IndexLabel({label, count}) {
    return e('div', {
        className: 'index-label'
    }, [
        e('span', { key: 'label'}, label),
        e('span', { key: 'count'}, `(${count})`),
    ]);
}

function IndexEntryLabel({entry}) {
    return e('span', {
        className: 'index-entry-label',
    }, `${entry.kind}`);
}

function IndexEntry({article, setArticle, entry, scrollState}) {
    const isActive = article.kind === entry.kind && article.id === entry.id;
    const [scrolled, setScrolled] = scrollState;

    useEffect(() => {
        if(isActive && !scrolled) {
            const div_id = `${entry.kind}_${entry.id}_idx`;
            const div = document.getElementById(div_id);
            if(div && div.classList.contains('index-entry')) {
                div.scrollIntoView();
                setScrolled(true);
            }
        }
    }, [isActive, scrolled]);

    return e('div', {
        className: 'index-entry' + (isActive ? ' active' : ''),
        id: `${entry.kind}_${entry.id}_idx`,
        onClick: () => { setArticle({ kind: entry.kind, id: entry.id}) }
    }, [
        e('span', {key: 'title' }, entry.title),
        e(IndexEntryLabel, {entry, key: "label"})
    ]);
}

function IndexSearch({searched, setSearched }) {
    return e('input', {
        type: 'text',
        className: 'index-search',
        placeholder: 'Search',
        value: searched,
        onChange: event => { setSearched(event.target.value); }
    });
}

function Index({article, setArticle, tab}) {
    const [index, setIndex] = useState([]);
    const [searched, setSearched] = useState("");
    const scrollState = useState(false);

    useEffect(() => {
        GetIndex(setIndex, tab[1], searched);
    }, [ tab, searched ]);

    const children = index.flatMap(group => [
        e(IndexLabel, {key: group[0], label: group[0], count: group[1].length }),
        ... group[1].map((entry, idx) =>
            e(IndexEntry, { scrollState, article, setArticle, entry, key: `${group[0]}-${idx}` }))
    ]);

    return e('div', { className: 'index' }, [
        e(IndexSearch, { searched, setSearched, key: '$search' }),
        ...children
    ]);
}

function SidebarTab({ label, isActive, onClick }) {
    return e('div', {
        className: 'sidebar-tab' + (isActive ? ' active' : ''),
        onClick: () => { onClick(); }
    }, label)
}

function SidebarTabbar({tab, setTab}) {
    const tabNames = ['abc', 'kind'];

    const buildTab = (label, idx, onClick) => e(SidebarTab, {
        label: label,
        key: label,
        onClick,
        isActive: idx === tab[0]
    });


    const tabs = tabNames.map((label, idx) =>
        buildTab(label, idx + 1, () => { setTab([idx + 1, label]); })
    );

    const children = [
        buildTab('recent', 0, () =>  {

        }),
        ...tabs,
        buildTab('hide', 1 + tabs.length, () =>  {

        }),
    ];

    return e('div', {
        className: 'sidebar-tabbar'
    }, children);
}

function LeftSidebar({article, setArticle}) {
    const [tab, setTab] = useState([1, 'abc']);

    return e('div', {
        className: 'left-sidebar',
    }, [
        e(SidebarTabbar, { key: "tabbar", tab, setTab, }),
        e(Index, {key:"idx", article, setArticle, tab }),
    ]);
}

function App() {
    const defaultArticle = { kind: 'index', id: '00000000'};
    const fromPath = path => {
        const members = path.split('/').slice(1);
        return members.length === 2 ? { kind: members[0], id: members[1] } : defaultArticle;
    };
    const [article, setArticle] = useState(fromPath(location.pathname));

    useEffect(() => {
        function interceptClickEvent(e) {
            const target = e.target || e.srcElement;
            if (target.tagName === 'A') {
                const href = target.getAttribute('href');

                if(href.startsWith("/")) {
                    console.log('Href', href);
                    e.preventDefault();
                    setArticle(fromPath(href));
                }
            }
        }

        if(document.addEventListener) {
            document.addEventListener('click', interceptClickEvent);
        } else if (document.attachEvent) {
            document.attachEvent('onclick', interceptClickEvent);
        }
    }, [true]);

    return e('div', { className: 'app' }, [
        e(LeftSidebar, { key: "left-sidebar", article, setArticle}),
        e(Article, {key: "article", ...article}),
    ]);
}

const domContainer = document.querySelector('#main');
let dom = e(App);
console.log('Dom', dom);
ReactDOM.render(dom, domContainer);
