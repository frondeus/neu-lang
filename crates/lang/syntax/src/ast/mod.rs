mod generated;
pub use generated::*;
use microtree::Ast;

impl MainArticle {
    pub fn get_articles(&self) -> impl Iterator<Item = SubArticle> {
        self.article_body()
            .into_iter()
            .flat_map(|a| {
                a.red()
                .pre_order()
                .filter_map(SubArticle::new)
                .collect::<Vec<_>>()
            })
    }
}

impl ArticleRef {
    pub fn item_ident(&self) -> Option<ItemIdent> {
        self.item_ident_token()
    }

    pub fn item_id(&self) -> Option<ArticleItemId> {
        self.article_item_id_token()
    }
}

impl ArticleItem {
    pub fn strukt(&self) -> Option<ArticleHeaderValues> {
        match self {
            ArticleItem::MainArticle(article) => {
                article.article_header()?
                    .article_header_values()
            }
            ArticleItem::SubArticle(article) => {
                article.sub_article_header()?
                    .article_header_values()
            }
        }
    }

    pub fn item_ident(&self) -> Option<ItemIdent> {
        match self {
            ArticleItem::MainArticle(article) => {
                article.article_header()?
                .item_ident_token()
            }
            ArticleItem::SubArticle(article) => {
                article.sub_article_header()?
                .item_ident_token()
            }
        }
    }

    pub fn item_id(&self) -> Option<ArticleItemId> {
        match self {
            ArticleItem::MainArticle(article) => {
                article.article_header()?
                .article_item_id_token()
            }
            ArticleItem::SubArticle(article) => {
                article.sub_article_header()?
                .article_item_id_token()
            }
        }
    }

    pub fn body(&self) -> Option<ArticleBody> {
        match self {
            ArticleItem::MainArticle(article) => article.article_body(),
            ArticleItem::SubArticle(article) => article.article_body()
        }
    }
}

impl Markdown {
    pub fn all_links(&self) -> impl Iterator<Item = MdLink> + '_ {
        self.0.pre_order()
            .filter_map(|e| MdLink::new(e))
    }
}
