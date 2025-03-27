# VkDocs-rs

VkDocs-rs is a supplementary crate for working with VK-Docs format ([see open-source VK Cloud documentation for more information](https://github.com/vk-cs/docs-public)). This crate helps to maintain [VkDocs meta files](https://github.com/vk-cs/docs-public/blob/master/guides/how-it-works.md) and update it correspondingly to the changes.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
vkdocs = "0.1"
```

## Example usage

Here's a basic usage of the VkDocs-rs crate.

```rust
use vkdocs::VkDocs;

// Create a documentation on a cool module.
let vkdocs = VkDocs::new("vkdoc-project");

let cool_module_path = Path::new("module/cool");
vkdocs.upsert(&cool_module_path, "Here as a documentation on a cool module.");

// Update the documentation on a cool module.
// The call automatically updates the metadata.
vkdocs.upsert(&cool_module_path, "Here as a newer documentation on a cool module.");
```

If you also want to generate template-based Markdown documentation files we recommend to use [Tera crate](https://github.com/Keats/tera).
