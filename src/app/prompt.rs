/// Generates the final prompt string.

pub fn generate_readme_prompt(
    description: &str,
    style: &str,
    extra_context: &str,
    repo_context: &str,
) -> String {
    let role = build_role_section(style);
    let task = build_task_section(description, extra_context);
    let context = build_context_section(repo_context);
    let requirements = build_requirements_section();

    format!("{}\n\n{}\n\n{}\n\n{}", role, task, context, requirements)
}

fn build_role_section(style: &str) -> String {
    format!(
        "# PROMPT FOR LLM: README GENERATION\n\n\
        ## 1. ROLE DEFINITION\n\
        You are an expert Technical Writer and Developer Advocate.\n\
        Your tone should be **{}**.\n\
        Your goal is to analyze the provided source code and generate a comprehensive, production-ready README.md file.",
        style
    )
}

fn build_task_section(description: &str, extra_context: &str) -> String {
    let mut section = format!(
        "## 2. USER REQUIREMENT\n\
        **Goal:** {}\n",
        description
    );

    if !extra_context.is_empty() {
        section.push_str(&format!(
            "\n**Specific Constraints:**\n- {}\n",
            extra_context
        ));
    }
    section
}

fn build_context_section(repo_context: &str) -> String {
    if repo_context.is_empty() {
        return String::new();
    }


    format!(
        "## 3. SOURCE CODE CONTEXT\n\
        The following is the actual file structure and content of the project. \
        Use this to derive installation steps, dependencies, and features.\n\n\
        ```xml\n\
        {}\n\
        ```",
        repo_context
    )
}


fn build_requirements_section() -> String {
    r#"## 4. OUTPUT REQUIREMENTS
    Please generate a single `README.md` file code block. Ensure the following sections are included (if applicable based on the code):
    
    1.  **Title & Badges:** Project name and relevant status badges (CI, License, version).
    2.  **Description:** A clear 'Elevator Pitch' based on the code's functionality.
    3.  **Features:** Bullet points extracted from the actual implemented logic.
    4.  **Tech Stack:** derived from `Cargo.toml`, `package.json`, etc.
    5.  **Prerequisites:** What needs to be installed (Rust, Node, etc).
    6.  **Installation:** Step-by-step commands.
    7.  **Usage:** Examples of how to run the tool (CLI flags, API calls).
    8.  **Configuration:** specific environment variables or config options found in the code.
    
    **Important Content Rule:** Do not include placeholder text like "Insert description here" - **infer it from the code provided.**

    ### **File Generation & Output Formatting Rule**

    When the user's request requires the generation of a file, a complete code snippet, or a document intended to be copied (like a system prompt or a configuration file), you must follow a specific output format.

    **The default output format is a self-contained HTML document that presents the raw source code within a `<textarea>` element.**

    This HTML document must include:

    1. **A Clear Header:** A title and brief description of the content.

    2. **A `<textarea>` Element:** This element must contain the complete, raw, un-rendered source code of the requested file. It should be set to `readonly`.

    3. **A "Copy to Clipboard" Button:** A prominent button that, when clicked, copies the entire content of the `<textarea>` to the user's clipboard.

    4. **User Feedback:** The copy functionality must provide clear visual feedback, such as changing the button text to "Copied!" for a few seconds. The JavaScript should be robust and compatible with the canvas environment.

    5. **Professional Styling:** The page must be styled using Tailwind CSS for a clean, modern, and usable interface.

    This rule should only be overridden if the user explicitly asks for a different format, such as "show me the rendered markdown" or "just give me the raw code block."
    
    ---
    *Begin by analyzing the code structure above, then generate the HTML-wrapped README.*"#
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_generation() {
        let role = build_role_section("Funny");
        assert!(role.contains("Funny"));
        assert!(role.contains("Technical Writer"));
    }

    #[test]
    fn test_task_generation() {
        let task = build_task_section("Make it pop", "No emojis");
        assert!(task.contains("Make it pop"));
        assert!(task.contains("No emojis"));
    }
}