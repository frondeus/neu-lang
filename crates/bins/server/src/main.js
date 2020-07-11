'use strict';

// API
function GetIndex(setIndex, by) {
    fetch('/neu/index.json')
        .then(response => response.json())
        .then(data => {
            if(!by) {
                setIndex(data.data);
            } else {
                const idx = data[by];
                let full_idx = [];
                for (const entry in idx) {
                    const entry_idx = idx[entry].map(i => data.data[i]);
                    full_idx.push([entry, entry_idx]);
                }
                setIndex(full_idx);
            }
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

function IndexLabel({label}) {
    return e('div', {
        className: 'index-label'
    }, label);
}

function IndexEntryLabel({entry}) {
    return e('span', {
        className: 'index-entry-label',
    }, `${entry.kind}`);
}

function IndexEntry({article, setArticle, entry}) {
    const isActive = article.kind === entry.kind && article.id === entry.id;
    return e('div', {
        className: 'index-entry' + (isActive ? ' active' : ''),
        onClick: () => { setArticle({ kind: entry.kind, id: entry.id}) }
    }, [
        e('span', {key: 'title' }, entry.title),
        e(IndexEntryLabel, {entry, key: "label"})
    ]);
}

function Index({article, setArticle, tab}) {
    const [index, setIndex] = useState([]);

    const once = true;

    useEffect(() => {
        GetIndex(setIndex, tab[1]);
    }, [ once, tab ]);

    const children = index.flatMap(group => [
        e(IndexLabel, {key: group[0], label: group[0] }),
        ... group[1].map((entry, idx) => e(IndexEntry, { article, setArticle, entry, key: `${group[0]}-${idx}` }))
    ]);

    return e('div', { className: 'index' }, children);
}

function SidebarTab({ label, isActive, idx, setTab }) {
    return e('div', {
        className: 'sidebar-tab' + (isActive ? ' active' : ''),
        onClick: () => {
            setTab([idx, label]);
        }
    }, label)
}

function SidebarTabbar({tab, setTab}) {
    const tabs = ['project', 'abc', 'kind', 'history'];

    const children = tabs.map((label, idx) =>
        e(SidebarTab, { label: label, key: label, setTab, idx, isActive: idx === tab[0] })
    );

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
    const path = location.pathname.split('/').slice(1);
    const defaultArticle = { kind: 'index', id: '00000000'};
    const initArticle = path.length === 2 ? { kind: path[0], id: path[1] } : defaultArticle;

    const [article, setArticle] = useState(initArticle);

    return e('div', { className: 'app' }, [
        e(LeftSidebar, { key: "left-sidebar", article, setArticle}),
        e(Article, {key: "article", ...article}),
    ]);
}

const domContainer = document.querySelector('#main');
let dom = e(App);
console.log('Dom', dom);
ReactDOM.render(dom, domContainer);
