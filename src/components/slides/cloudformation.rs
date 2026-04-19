use leptos::*;
use crate::i18n::get_translation;

const CODE: &str = r#"AWSTemplateFormatVersion: '2010-09-09'
Description: S3 bucket with versioning and server-side encryption

Parameters:
  BucketName:
    Type:    String
    Default: my-app-assets

Resources:
  AppBucket:
    Type: AWS::S3::Bucket
    Properties:
      BucketName: !Ref BucketName
      VersioningConfiguration:
        Status: Enabled
      BucketEncryption:
        ServerSideEncryptionConfiguration:
          - ServerSideEncryptionByDefault:
              SSEAlgorithm: AES256
      PublicAccessBlockConfiguration:
        BlockPublicAcls:       true
        BlockPublicPolicy:     true
        IgnorePublicAcls:      true
        RestrictPublicBuckets: true

Outputs:
  BucketArn:
    Value:       !GetAtt AppBucket.Arn
    Description: ARN of the created S3 bucket
    Export:
      Name: !Sub "${AWS::StackName}-BucketArn""#;

#[component]
pub fn CloudFormationSlide() -> impl IntoView {
    let t = get_translation;
    let show_code = create_rw_signal(false);

    view! {
        <div class="slide tool-slide" style="--tool-color: #e67e22">
            <div class="slide-header" style="border-top: 4px solid #e67e22">
                <div class="slide-header-left">
                    <h2 class="slide-title">{move || t("cf.title")}</h2>
                    <span class="slide-subtitle">{move || t("cf.subtitle")}</span>
                </div>
                <div class="slide-header-right">
                    <span class="slide-year-badge" style="background: #e67e22">"2011"</span>
                    <span class="slide-category-badge">{move || t("slide.cloud_provisioning")}</span>
                    <span class="slide-creator">{move || t("cf.creator")}</span>
                </div>
            </div>

            <div class="slide-tab-bar">
                <button class="slide-tab"
                    class:active=move || !show_code.get()
                    on:click=move |_| show_code.set(false)>
                    "Lore"
                </button>
                <button class="slide-tab"
                    class:active=move || show_code.get()
                    on:click=move |_| show_code.set(true)>
                    "Code"
                </button>
            </div>

            <div class="slide-body" style:display=move || if show_code.get() { "none" } else { "" }>
                <div class="aspects-col">
                    <h3 class="col-heading">{move || t("slide.key_aspects")}</h3>
                    <ul class="aspects-list">
                        <li>{move || t("cf.aspect.1")}</li>
                        <li>{move || t("cf.aspect.2")}</li>
                        <li>{move || t("cf.aspect.3")}</li>
                        <li>{move || t("cf.aspect.4")}</li>
                        <li>{move || t("cf.aspect.5")}</li>
                        <li>{move || t("cf.aspect.6")}</li>
                    </ul>
                </div>

                <div class="diagram-col">
                    <h3 class="col-heading">{move || t("cf.diagram.title")}</h3>
                    <svg viewBox="0 0 500 330" class="flow-diagram" xmlns="http://www.w3.org/2000/svg">
                        <rect width="500" height="330" fill="#111827" rx="10"/>

                        // Template file
                        <rect x="150" y="15" width="200" height="46" rx="8"
                              fill="#1e2d45" stroke="#e67e22" stroke-width="1.5"/>
                        <text x="250" y="35" text-anchor="middle" class="diag-box-title" fill="#e67e22">
                            "template.yaml"
                        </text>
                        <text x="250" y="53" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "JSON or YAML template"
                        </text>

                        // Arrow
                        <line x1="250" y1="61" x2="250" y2="100"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="244,95 256,95 250,105" fill="#4a5580"/>

                        // CloudFormation API
                        <rect x="90" y="105" width="320" height="70" rx="8"
                              fill="#1e2d45" stroke="#e67e22" stroke-width="1.5"/>
                        <text x="250" y="125" text-anchor="middle" class="diag-box-title" fill="#e67e22">
                            "CloudFormation API"
                        </text>
                        <text x="250" y="143" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "Validate → Compute Change Set"
                        </text>
                        <text x="250" y="162" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "Execute stack (create / update)"
                        </text>

                        // Arrow
                        <line x1="250" y1="175" x2="250" y2="210"
                              stroke="#4a5580" stroke-width="1.5"/>
                        <polygon points="244,205 256,205 250,215" fill="#4a5580"/>

                        // AWS Resources fan
                        <rect x="20"  y="215" width="130" height="40" rx="6"
                              fill="#162030" stroke="#e67e22" stroke-width="1"/>
                        <text x="85"  y="240" text-anchor="middle" class="diag-box-sub" fill="#68d391">
                            "VPC / Subnets"
                        </text>

                        <rect x="185" y="215" width="130" height="40" rx="6"
                              fill="#162030" stroke="#e67e22" stroke-width="1"/>
                        <text x="250" y="240" text-anchor="middle" class="diag-box-sub" fill="#68d391">
                            "EC2 / RDS"
                        </text>

                        <rect x="350" y="215" width="130" height="40" rx="6"
                              fill="#162030" stroke="#e67e22" stroke-width="1"/>
                        <text x="415" y="240" text-anchor="middle" class="diag-box-sub" fill="#68d391">
                            "S3 / IAM"
                        </text>

                        // Fan lines
                        <line x1="250" y1="215" x2="85" y2="215" stroke="#4a5580" stroke-width="1"/>
                        <line x1="250" y1="215" x2="415" y2="215" stroke="#4a5580" stroke-width="1"/>

                        // Stack state banner
                        <rect x="60" y="275" width="380" height="40" rx="6"
                              fill="#0d1117" stroke="#2d3f6b" stroke-width="1"/>
                        <text x="250" y="291" text-anchor="middle" class="diag-box-sub" fill="#8898b8">
                            "CloudFormation tracks stack state internally"
                        </text>
                        <text x="250" y="308" text-anchor="middle" class="diag-box-sub" fill="#6b7cb8">
                            "Status: CREATE_COMPLETE / UPDATE_COMPLETE"
                        </text>
                    </svg>
                </div>
            </div>

            <div class="code-section code-section-expanded" style:display=move || if !show_code.get() { "none" } else { "" }>
                <h4 class="code-caption">{move || t("cf.code.caption")}</h4>
                <pre class="code-block"><code class="language-yaml">{CODE}</code></pre>
            </div>
        </div>
    }
}
