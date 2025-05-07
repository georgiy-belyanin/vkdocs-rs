# VkDocs-rs

VkDocs-rs is a supplementary crate for working with VK-Docs format ([see open-source VK Cloud documentation for more information](https://github.com/vk-cs/docs-public)). This crate helps to maintain [VkDocs meta files](https://github.com/vk-cs/docs-public/blob/master/guides/how-it-works.md) and update it correspondingly to the changes.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
vkdocs-rs = "0.1"
```

## Example usage

Here's a basic usage of the VkDocs-rs crate.

```rust
use vkdocs_rs::VkDocs;

// Create a documentation on a cool module.
let vkdocs = VkDocs::new("vkdoc-project");

// Describe a new VK Doc article.
let page = Page::new()
    .with_title("Cool module".to_string())
    .with_content("Docs on a cool module.".to_string());

// Create a new article based on the provided page information.
vkdocs.upsert(&cool_module_path, page);

// Update the article with newer contents.
let page = Page::new()
    .with_content("Newer docs on a cool module.".to_string());

// This call also automatically updates the article's metadata.
vkdocs.upsert(&cool_module_path, page);
```

If you also want to generate template-based Markdown documentation files we recommend to use [Tera crate](https://github.com/Keats/tera).
