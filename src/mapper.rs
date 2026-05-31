use pulldown_cmark::{Event, Parser};

use crate::dto::*;
use crate::model::*;
// use bson::DateTime as BDT;
// use chrono::{DateTime, Utc};

// タグリストのデータ変換
// Optional<Vec<TagDTO>> -> Optional<Vec<Tag>>
pub fn to_tags(tags: Option<Vec<TagDTO>>) -> Option<Vec<Tag>> {
    match tags {
        Some(tags) => Some(
            tags.into_iter()
                .map(|tag| Tag {
                    name: Name {
                        ja: tag.name.ja,
                        en: tag.name.en,
                    },
                    slug: tag.slug,
                })
                .collect(),
        ),
        None => None,
    }
}

// タグリストのデータ変換
// Optional<Vec<Tag>> -> Optional<Vec<TagDTO>>
pub fn to_tags_dto(tags: Option<Vec<Tag>>) -> Option<Vec<TagDTO>> {
    match tags {
        Some(tags) => Some(
            tags.into_iter()
                .map(|tag| TagDTO {
                    name: NameDTO {
                        ja: tag.name.ja,
                        en: tag.name.en,
                    },
                    slug: tag.slug,
                })
                .collect(),
        ),
        None => None,
    }
}

/// タグリストのデータ変換
///
/// この関数は、`Option<Vec<Tag>>` 型のタグリストを、`Option<Vec<TagResponseDTO>>` 型に変換します。
/// `lang` パラメータに基づいて、タグの名前を日本語（`Lang::Ja`）または英語（`Lang::En`）に変換します。
///
/// # 引数
/// - `tags`: タグのリスト (`Option<Vec<Tag>>`)。`None` の場合は変換せず `None` を返します。
/// - `lang`: 言語の選択 (`Lang` 型)。
///
/// # 戻り値
/// - タグリストが存在する場合は、それぞれのタグを変換した後、`Some(Vec<TagResponseDTO>)` を返します。
/// - タグリストが `None` の場合は `None` を返します。
///
pub fn to_response_tag_dto(tags: Option<Vec<Tag>>, lang: &Lang) -> Option<Vec<ResponseTagDTO>> {
    match tags {
        Some(items) => Some(
            items
                .into_iter()
                .map(|tag| ResponseTagDTO {
                    slug: tag.slug,
                    name: match lang {
                        Lang::Ja => tag.name.ja,
                        Lang::En => tag.name.en,
                    },
                })
                .collect(),
        ),
        None => None,
    }
}

// カテゴリのデータ変換
// CategoryDTO -> Category
pub fn to_category(category: &CategoryDTO) -> Category {
    Category {
        name: Name {
            ja: category.name.ja.clone(),
            en: category.name.en.clone(),
        },
        slug: category.slug.clone(),
    }
}

// カテゴリのデータ変換
// Category -> ResponseCategoryDTO
pub fn to_response_category_dto(category: Category, lang: &Lang) -> ResponseCategoryDTO {
    ResponseCategoryDTO {
        slug: category.slug,
        name: match lang {
            Lang::Ja => category.name.ja,
            Lang::En => category.name.en,
        },
    }
}

// 記事のデータ変換
// Post -> PostListDTO
pub fn to_post_list_dto(post: Post, lang: &Lang) -> PostListDTO {
    let excerpt_len: usize = 120;
    PostListDTO {
        id: match post.id {
            Some(id) => id.to_string(),
            None => "".to_string(),
        },
        title: match lang {
            Lang::Ja => post.title.ja,
            Lang::En => post.title.en,
        },
        slug: post.slug,
        is_draft: post.is_draft,
        has_english: post.has_english,
        excerpt: create_excerpt(
            match lang {
                Lang::Ja => post.content.ja.as_str(),
                Lang::En => post.content.en.as_str(),
            },
            excerpt_len,
        ),
        category: to_response_category_dto(post.category, lang),
        tags: to_response_tag_dto(post.tags, lang),
        published_at: match post.published_at {
            None => None,
            Some(d) => Some(d.to_chrono()),
        },
        updated_at: post.updated_at.to_chrono(),
        image: post.image,
        author_id: post.author_id.to_string(),
    }
}

// 記事データの抜粋を作成
fn create_excerpt(markdown: &str, max_len: usize) -> String {
    let parser = Parser::new(markdown);
    let mut text = String::new();

    for event in parser {
        match event {
            Event::Text(body) | Event::Code(body) => {
                text.push_str(&body);
            }
            // 改行などはスペースや適当な区切りに変換
            Event::SoftBreak | Event::HardBreak => {
                text.push(' ');
            }
            _ => {}
        }
    }

    // 不要な連続スペースの圧縮やトリム
    let clean_text = text.split_whitespace().collect::<Vec<_>>().join(" ");

    if clean_text.chars().count() > max_len {
        clean_text.chars().take(max_len).collect::<String>() + "..."
    } else {
        clean_text
    }
}

// 記事のデータ変換
// PostDetail -> PostDetailDTO
pub fn to_post_dto(post: PostDetail, lang: &Lang) -> PostDetailDTO {
    let excerpt_len: usize = 120;
    PostDetailDTO {
        id: post.id.map(|id| id.to_hex()),
        title: {
            match lang {
                Lang::Ja => post.title.ja,
                Lang::En => post.title.en,
            }
        },
        lang: {
            match lang {
                Lang::Ja => "ja".to_string(),
                Lang::En => "en".to_string(),
            }
        },
        slug: post.slug,
        is_draft: post.is_draft,
        has_english: post.has_english,
        category: to_response_category_dto(post.category, lang),
        tags: to_response_tag_dto(post.tags, lang),
        published_at: match post.published_at {
            None => None,
            Some(d) => Some(d.to_chrono()),
        },
        updated_at: post.updated_at.to_chrono(),
        author: to_author_dto(post.author, &lang),
        image: post.image,
        excerpt: create_excerpt(
            match lang {
                Lang::Ja => post.content.ja.as_str(),
                Lang::En => post.content.en.as_str(),
            },
            excerpt_len,
        ),
        content: {
            match lang {
                Lang::Ja => post.content.ja,
                Lang::En => post.content.en,
            }
        },
    }
}

// 管理者用の記事DTOへの変換
pub fn to_admin_post_dto(post: Post) -> AdminPostDTO {
    AdminPostDTO {
        id: match post.id {
            Some(i) => Some(i.to_string()),
            None => None,
        },
        title: NameDTO {
            ja: post.title.ja,
            en: post.title.en,
        },
        slug: post.slug,
        is_draft: post.is_draft,
        has_english: post.has_english,
        category: CategoryDTO {
            slug: post.category.slug,
            name: NameDTO {
                ja: post.category.name.ja,
                en: post.category.name.en,
            },
        },
        tags: to_tags_dto(post.tags),
        published_at: match post.published_at {
            Some(d) => Some(d.to_chrono()),
            None => None,
        },
        updated_at: post.updated_at.to_chrono(),
        created_at: post.created_at.to_chrono(),
        author_id: post.author_id.to_string(),
        image: post.image,
        content: ContentDTO {
            ja: post.content.ja,
            en: post.content.en,
        },
    }
}

// 著者データの変換
// Author -> AuthorDTO
fn to_author_dto(author: Option<Author>, lang: &Lang) -> Option<AuthorDTO> {
    match author {
        Some(a) => Some(AuthorDTO {
            id: a.id.to_string(),
            name: match lang {
                Lang::Ja => a.name.ja,
                Lang::En => a.name.en,
            },
        }),
        None => None,
    }
}

pub enum Lang {
    Ja,
    En,
}

// ユーザーDTOへの変換
pub fn to_user_dto(user: &User) -> UserDTO {
    UserDTO {
        id: None,
        name: NameDTO {
            ja: user.name.ja.clone(),
            en: user.name.en.clone(),
        },
        email: user.email.clone(),
        role: user.role.clone(),
        password: "******".to_owned(),
    }
}

// レスポンス用ユーザーDTOへの変換
pub fn to_user_response_dto(user: &User) -> UserResponseDTO {
    UserResponseDTO {
        id: None,
        name: NameDTO {
            ja: user.name.ja.clone(),
            en: user.name.en.clone(),
        },
        email: user.email.clone(),
        role: user.role.clone(),
    }
}

// ページリストのデータ変換
// Vec<Page> -> Vec<PageListDTO>
pub fn to_page_list_dto(pages: Vec<Page>) -> Vec<PageListDTO> {
    pages
        .into_iter()
        .map(|page| PageListDTO {
            id: match page.id {
                Some(id) => id.to_string(),
                None => "".to_string(),
            },
            slug: page.slug,
            title: NameDTO {
                ja: page.title.ja,
                en: page.title.en,
            },
            created_at: page.created_at.to_chrono(),
            updated_at: page.updated_at.to_chrono(),
        })
        .collect()
}

// ページ詳細データDTOへの変換
pub fn to_page_detail_dto(page: Page, lang: &Lang) -> PageDetailDTO {
    PageDetailDTO {
        slug: page.slug,
        title: match lang {
            Lang::Ja => page.title.ja,
            Lang::En => page.title.en,
        },
        content: match lang {
            Lang::Ja => page.content.ja,
            Lang::En => page.content.en,
        },
        created_at: page.created_at.to_chrono(),
        updated_at: page.updated_at.to_chrono(),
    }
}

// 管理画面用のページ詳細DTOへの変換
pub fn to_admin_page_detail_dto(page: Page) -> AdminPageDetailDTO {
    AdminPageDetailDTO {
        id: match page.id {
            Some(id) => id.to_string(),
            None => "".to_string(),
        },
        slug: page.slug,
        title: NameDTO {
            ja: page.title.ja,
            en: page.title.en,
        },
        content: ContentDTO {
            ja: page.content.ja,
            en: page.content.en,
        },
        created_at: page.created_at.to_chrono(),
        updated_at: page.updated_at.to_chrono(),
    }
}

// CategoryWithCount -> SSGDataWithCountDTOの変換
impl From<TagWithCount> for SSGDataWithCountDTO {
    fn from(item: TagWithCount) -> Self {
        SSGDataWithCountDTO {
            slug: item.slug,
            count: item.count,
        }
    }
}

// TagWithCount -> ssgdatawithcountdtoの変換
impl From<CategoryWithCount> for SSGDataWithCountDTO {
    fn from(item: CategoryWithCount) -> Self {
        SSGDataWithCountDTO {
            slug: item.slug,
            count: item.count,
        }
    }
}

// ssgdatawithcountdtoの変換
pub fn to_ssg_data_with_count_dto<T>(data: Vec<T>) -> Vec<SSGDataWithCountDTO>
where
    SSGDataWithCountDTO: From<T>,
{
    data.into_iter().map(|item| item.into()).collect()
}
