pub fn for_locale(locale: &str) -> &'static [&'static str] {
    match locale {
        "de-DE" => DE,
        _ => EN,
    }
}

pub fn comma_separated(locale: &str) -> String {
    for_locale(locale).join(", ")
}

const EN: &[&str] = &[
    "Backend Developer",
    "DevOps Engineer",
    "Full Stack Developer",
    "Software Engineer",
    "API Development",
    "Microservices",
    "REST API",
    "GraphQL",
    "gRPC",
    "Node.js",
    "TypeScript",
    "JavaScript",
    "Python",
    "Go",
    "Rust",
    "Docker",
    "Kubernetes",
    "Helm",
    "Terraform",
    "Ansible",
    "AWS",
    "Azure",
    "Google Cloud",
    "DigitalOcean",
    "Vercel",
    "PostgreSQL",
    "MongoDB",
    "Redis",
    "Elasticsearch",
    "MySQL",
    "CI/CD",
    "Jenkins",
    "GitHub Actions",
    "GitLab CI",
    "ArgoCD",
    "Monitoring",
    "Prometheus",
    "Grafana",
    "ELK Stack",
    "Observability",
    "Load Balancing",
    "Nginx",
    "Traefik",
    "Service Mesh",
    "Istio",
    "Agile",
    "Scrum",
    "Clean Architecture",
    "TDD",
    "DDD",
    "Cloud Native",
    "Infrastructure as Code",
    "GitOps",
    "Software Developer",
    "Freelancer",
    "Remote Developer",
    "Cloud Consultant",
];

const DE: &[&str] = &[
    "Backend Entwickler",
    "DevOps Engineer",
    "Full Stack Developer",
    "Software Engineer",
    "API Entwicklung",
    "Microservices",
    "REST API",
    "GraphQL",
    "gRPC",
    "Node.js",
    "TypeScript",
    "JavaScript",
    "Python",
    "Go",
    "Rust",
    "Docker",
    "Kubernetes",
    "Helm",
    "Terraform",
    "Ansible",
    "AWS",
    "Azure",
    "Google Cloud",
    "DigitalOcean",
    "Vercel",
    "PostgreSQL",
    "MongoDB",
    "Redis",
    "Elasticsearch",
    "MySQL",
    "CI/CD",
    "Jenkins",
    "GitHub Actions",
    "GitLab CI",
    "ArgoCD",
    "Monitoring",
    "Prometheus",
    "Grafana",
    "ELK Stack",
    "Observability",
    "Load Balancing",
    "Nginx",
    "Traefik",
    "Service Mesh",
    "Istio",
    "Agile",
    "Scrum",
    "Clean Architecture",
    "TDD",
    "DDD",
    "Cloud Native",
    "Infrastructure as Code",
    "GitOps",
    "Software Entwickler",
    "Freelancer",
    "Remote Developer",
    "Cloud Consultant",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn en_us_returns_english_keywords() {
        assert!(for_locale("en-US").contains(&"Backend Developer"));
        assert!(!for_locale("en-US").contains(&"Backend Entwickler"));
    }

    #[test]
    fn de_de_returns_german_keywords() {
        assert!(for_locale("de-DE").contains(&"Backend Entwickler"));
    }

    #[test]
    fn unknown_locale_falls_back_to_english() {
        assert_eq!(for_locale("fr-FR"), for_locale("en-US"));
    }

    #[test]
    fn comma_separated_joins_with_comma_space() {
        let joined = comma_separated("en-US");
        assert!(joined.contains("Backend Developer, "));
    }
}
