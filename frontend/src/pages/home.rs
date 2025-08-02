// Homepage Component - 2025 Design System
// Modern homepage with bento grid layout and dopamine colors

use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::layout::{BentoGrid, BentoCard, BentoSection};
use crate::components::ui::{GradientButton, StatusCard, QuickAction};
use crate::components::ui::{ButtonVariant, ButtonSize, StatusCardVariant, TrendDirection};
use crate::styles::{use_theme, Typography, TypographyVariant, TypographyColor};

#[derive(Properties, PartialEq)]
pub struct HomePageProps {
    pub user_name: Option<String>,
    pub farm_stats: Option<FarmStats>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FarmStats {
    pub total_diagnoses: u32,
    pub health_score: f32,
    pub recent_diagnoses: u32,
    pub active_treatments: u32,
}

impl Default for FarmStats {
    fn default() -> Self {
        Self {
            total_diagnoses: 15,
            health_score: 92.0,
            recent_diagnoses: 3,
            active_treatments: 2,
        }
    }
}

#[function_component(HomePage)]
pub fn home_page(props: &HomePageProps) -> Html {
    let theme = use_theme();
    let colors = &theme.colors;

    let user_name = props.user_name.as_deref().unwrap_or("เกษตรกร");
    let farm_stats = props.farm_stats.as_ref().unwrap_or(&FarmStats::default());

    // Navigation callbacks
    let navigate_to_camera = Callback::from(|_| {
        // Navigate to camera page
        web_sys::console::log_1(&"Navigate to camera".into());
    });

    let navigate_to_chat = Callback::from(|_| {
        // Navigate to chat page
        web_sys::console::log_1(&"Navigate to chat".into());
    });

    let navigate_to_history = Callback::from(|_| {
        // Navigate to diagnosis history
        web_sys::console::log_1(&"Navigate to history".into());
    });

    let navigate_to_profile = Callback::from(|_| {
        // Navigate to profile
        web_sys::console::log_1(&"Navigate to profile".into());
    });

    html! {
        <div class="home-page">
            // Hero Section with Gradient Background
            <section class="hero-section">
                <div class="hero-content">
                    <div class="hero-greeting">
                        <Typography variant={TypographyVariant::H1} class="hero-title">
                            {"🌾 AI4Thai Crop Guardian"}
                        </Typography>
                        <Typography variant={TypographyVariant::Body1} class="hero-subtitle thai-text">
                            {format!("สวัสดี {}! ระบบตรวจจับโรคพืชด้วย AI สำหรับเกษตรกรไทย", user_name)}
                        </Typography>
                    </div>

                    <div class="hero-cta">
                        <GradientButton
                            variant={ButtonVariant::Primary}
                            size={ButtonSize::Large}
                            onclick={navigate_to_camera}
                            icon="📷"
                            full_width={true}
                        >
                            {"เริ่มตรวจวินิจฉัยโรคพืช"}
                        </GradientButton>
                    </div>
                </div>

                // Decorative background elements
                <div class="hero-bg-elements">
                    <div class="hero-bg-circle hero-bg-circle-1"></div>
                    <div class="hero-bg-circle hero-bg-circle-2"></div>
                    <div class="hero-bg-circle hero-bg-circle-3"></div>
                </div>
            </section>

            // Dashboard Section with Bento Grid
            <BentoSection
                title="แดชบอร์ดฟาร์ม"
                subtitle="ภาพรวมสุขภาพพืชผลและการจัดการฟาร์มของคุณ"
                class="dashboard-section"
            >
                <BentoGrid columns={4} gap="1.5rem" responsive={true}>
                    // Quick Camera Action - Large Card (2x2)
                    <BentoCard
                        span_cols={2}
                        span_rows={2}
                        gradient={colors.get_primary_gradient()}
                        hover_effect={true}
                        clickable={true}
                        onclick={navigate_to_camera}
                        class="camera-action-card"
                    >
                        <div class="quick-action-content">
                            <div class="quick-action-icon">{"📷"}</div>
                            <Typography variant={TypographyVariant::H3} color={TypographyColor::Inverse}>
                                {"ถ่ายรูปตรวจโรค"}
                            </Typography>
                            <Typography variant={TypographyVariant::Body2} color={TypographyColor::Inverse}>
                                {"ตรวจจับโรคพืชใน 3 วินาที"}
                            </Typography>
                            <div class="quick-action-badge">
                                {"🚀 ความแม่นยำ 98%"}
                            </div>
                        </div>
                    </BentoCard>

                    // Recent Diagnoses Stats
                    <BentoCard span_cols={1} span_rows={1} color={colors.accent_yellow}>
                        <StatusCard
                            title="การวินิจฉัยล่าสุด"
                            value={farm_stats.recent_diagnoses.to_string()}
                            trend={Some("+3 วันนี้".to_string())}
                            trend_direction={Some(TrendDirection::Up)}
                            icon={Some("📊".to_string())}
                            variant={StatusCardVariant::Custom(colors.primary_vibrant_orange.to_string())}
                            onclick={Some(navigate_to_history)}
                        />
                    </BentoCard>

                    // Farm Health Score
                    <BentoCard span_cols={1} span_rows={1} color={colors.accent_lime_green}>
                        <StatusCard
                            title="คะแนนสุขภาพฟาร์ม"
                            value={format!("{}%", farm_stats.health_score)}
                            trend={Some("ดีมาก".to_string())}
                            trend_direction={Some(TrendDirection::Up)}
                            icon={Some("🌱".to_string())}
                            variant={StatusCardVariant::Success}
                        />
                    </BentoCard>

                    // Chat Assistant - Wide Card (2x1)
                    <BentoCard
                        span_cols={2}
                        span_rows={1}
                        gradient={format!("linear-gradient(135deg, {}, {})", colors.accent_purple, colors.primary_energetic_pink)}
                        hover_effect={true}
                        clickable={true}
                        onclick={navigate_to_chat}
                    >
                        <div class="chat-action-content">
                            <div class="chat-action-icon">{"💬"}</div>
                            <div class="chat-action-text">
                                <Typography variant={TypographyVariant::H4} color={TypographyColor::Inverse}>
                                    {"ปรึกษาผู้เชี่ยวชาญ AI"}
                                </Typography>
                                <Typography variant={TypographyVariant::Body2} color={TypographyColor::Inverse}>
                                    {"สอบถามเรื่องการเกษตร ภาษาไทย"}
                                </Typography>
                            </div>
                            <div class="chat-action-arrow">{"→"}</div>
                        </div>
                    </BentoCard>

                    // Weather Widget
                    <BentoCard span_cols={1} span_rows={1} color={colors.primary_electric_blue}>
                        <WeatherWidget />
                    </BentoCard>

                    // Tips of the Day
                    <BentoCard span_cols={1} span_rows={1} color={colors.primary_vibrant_orange}>
                        <TipsCard />
                    </BentoCard>
                </BentoGrid>
            </BentoSection>

            // Quick Actions Section
            <BentoSection
                title="การดำเนินการด่วน"
                subtitle="เข้าถึงฟีเจอร์หลักได้อย่างรวดเร็ว"
                class="quick-actions-section"
            >
                <BentoGrid columns={3} gap="1rem">
                    <BentoCard hover_effect={true} clickable={true} onclick={navigate_to_history}>
                        <QuickAction
                            icon="📋"
                            title="ประวัติการวินิจฉัย"
                            subtitle={Some(format!("{} รายการ", farm_stats.total_diagnoses))}
                            onclick={navigate_to_history}
                            variant={StatusCardVariant::Info}
                        />
                    </BentoCard>

                    <BentoCard hover_effect={true} clickable={true} onclick={navigate_to_profile}>
                        <QuickAction
                            icon="👤"
                            title="โปรไฟล์ฟาร์ม"
                            subtitle={Some("จัดการข้อมูลฟาร์ม".to_string())}
                            onclick={navigate_to_profile}
                            variant={StatusCardVariant::Default}
                        />
                    </BentoCard>

                    <BentoCard hover_effect={true}>
                        <QuickAction
                            icon="📚"
                            title="คู่มือการเกษตร"
                            subtitle={Some("เคล็ดลับและวิธีการ".to_string())}
                            onclick={Callback::from(|_| {})}
                            variant={StatusCardVariant::Success}
                        />
                    </BentoCard>
                </BentoGrid>
            </BentoSection>

            // Recent Activity Section
            <BentoSection
                title="กิจกรรมล่าสุด"
                class="recent-activity-section"
            >
                <BentoGrid columns={1}>
                    <BentoCard>
                        <RecentActivityList />
                    </BentoCard>
                </BentoGrid>
            </BentoSection>
        </div>
    }
}

// Weather Widget Component
#[function_component(WeatherWidget)]
fn weather_widget() -> Html {
    html! {
        <div class="weather-widget">
            <div class="weather-icon">{"☀️"}</div>
            <div class="weather-info">
                <Typography variant={TypographyVariant::H6} color={TypographyColor::Inverse}>
                    {"เชียงใหม่"}
                </Typography>
                <Typography variant={TypographyVariant::H4} color={TypographyColor::Inverse}>
                    {"28°C"}
                </Typography>
                <Typography variant={TypographyVariant::Caption} color={TypographyColor::Inverse}>
                    {"แสงแดดดี เหมาะปลูกพืช"}
                </Typography>
            </div>
        </div>
    }
}

// Tips Card Component
#[function_component(TipsCard)]
fn tips_card() -> Html {
    html! {
        <div class="tips-card">
            <div class="tips-icon">{"💡"}</div>
            <div class="tips-content">
                <Typography variant={TypographyVariant::H6} color={TypographyColor::Inverse}>
                    {"เคล็ดลับวันนี้"}
                </Typography>
                <Typography variant={TypographyVariant::Body2} color={TypographyColor::Inverse} class="thai-text">
                    {"รดน้ำตอนเช้าจะช่วยให้พืชดูดซึมน้ำได้ดีกว่าตอนกลางวัน"}
                </Typography>
            </div>
        </div>
    }
}

// Recent Activity List Component
#[function_component(RecentActivityList)]
fn recent_activity_list() -> Html {
    let activities = vec![
        ("📷", "ตรวจพบโรคใบไหม้ข้าว", "2 ชั่วโมงที่แล้ว", "high"),
        ("💬", "ปรึกษาเรื่องการใส่ปุ๋ย", "5 ชั่วโมงที่แล้ว", "medium"),
        ("📊", "อัปเดตคะแนนสุขภาพฟาร์ม", "1 วันที่แล้ว", "low"),
        ("🌱", "เพิ่มข้อมูลพืชผลใหม่", "2 วันที่แล้ว", "low"),
    ];

    html! {
        <div class="recent-activity-list">
            <Typography variant={TypographyVariant::H5} class="activity-title">
                {"กิจกรรมล่าสุด"}
            </Typography>
            <div class="activity-items">
                { for activities.iter().map(|(icon, title, time, priority)| {
                    html! {
                        <div class={classes!("activity-item", format!("activity-{}", priority))}>
                            <div class="activity-icon">{icon}</div>
                            <div class="activity-content">
                                <Typography variant={TypographyVariant::Body1} class="activity-text thai-text">
                                    {title}
                                </Typography>
                                <Typography variant={TypographyVariant::Caption} color={TypographyColor::Secondary}>
                                    {time}
                                </Typography>
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

// CSS for homepage components
pub fn generate_homepage_css() -> String {
    r#"/* Homepage Styles - 2025 Design */

.home-page {
  min-height: 100vh;
  background: var(--color-bg-light);
}

/* Hero Section */
.hero-section {
  position: relative;
  background: var(--gradient-hero);
  padding: var(--space-2xl) var(--space-lg);
  margin-bottom: var(--space-2xl);
  overflow: hidden;
  border-radius: 0 0 var(--radius-2xl) var(--radius-2xl);
}

.hero-content {
  position: relative;
  z-index: 2;
  max-width: 800px;
  margin: 0 auto;
  text-align: center;
}

.hero-title {
  color: var(--color-text-inverse);
  margin-bottom: var(--space-md);
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.hero-subtitle {
  color: var(--color-text-inverse);
  font-size: var(--text-lg);
  margin-bottom: var(--space-xl);
  opacity: 0.9;
}

.hero-cta {
  max-width: 400px;
  margin: 0 auto;
}

/* Hero Background Elements */
.hero-bg-elements {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 1;
}

.hero-bg-circle {
  position: absolute;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.1);
  animation: float 6s ease-in-out infinite;
}

.hero-bg-circle-1 {
  width: 200px;
  height: 200px;
  top: 10%;
  right: 10%;
  animation-delay: 0s;
}

.hero-bg-circle-2 {
  width: 150px;
  height: 150px;
  bottom: 20%;
  left: 15%;
  animation-delay: 2s;
}

.hero-bg-circle-3 {
  width: 100px;
  height: 100px;
  top: 60%;
  right: 30%;
  animation-delay: 4s;
}

@keyframes float {
  0%, 100% { transform: translateY(0px); }
  50% { transform: translateY(-20px); }
}

/* Dashboard Section */
.dashboard-section {
  margin-bottom: var(--space-2xl);
}

/* Camera Action Card */
.camera-action-card {
  background: var(--gradient-primary) !important;
  color: var(--color-text-inverse);
}

.quick-action-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  height: 100%;
  gap: var(--space-md);
}

.quick-action-icon {
  font-size: 3rem;
  margin-bottom: var(--space-sm);
}

.quick-action-badge {
  background: rgba(255, 255, 255, 0.2);
  padding: var(--space-xs) var(--space-sm);
  border-radius: var(--radius-full);
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
}

/* Chat Action Card */
.chat-action-content {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  height: 100%;
}

.chat-action-icon {
  font-size: 2rem;
  flex-shrink: 0;
}

.chat-action-text {
  flex: 1;
}

.chat-action-arrow {
  font-size: 1.5rem;
  opacity: 0.8;
  transition: transform 0.3s ease;
}

.chat-action-content:hover .chat-action-arrow {
  transform: translateX(4px);
}

/* Weather Widget */
.weather-widget {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  height: 100%;
  color: var(--color-text-inverse);
}

.weather-icon {
  font-size: 2rem;
}

.weather-info {
  flex: 1;
}

/* Tips Card */
.tips-card {
  display: flex;
  gap: var(--space-md);
  height: 100%;
  color: var(--color-text-inverse);
}

.tips-icon {
  font-size: 1.5rem;
  flex-shrink: 0;
}

.tips-content {
  flex: 1;
}

/* Quick Actions Section */
.quick-actions-section {
  margin-bottom: var(--space-2xl);
}

/* Recent Activity */
.recent-activity-section {
  margin-bottom: var(--space-2xl);
}

.recent-activity-list {
  padding: var(--space-lg);
}

.activity-title {
  margin-bottom: var(--space-lg);
  color: var(--color-text-primary);
}

.activity-items {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.activity-item {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: var(--space-md);
  border-radius: var(--radius-lg);
  transition: background-color 0.2s ease;
}

.activity-item:hover {
  background: var(--color-bg-light);
}

.activity-icon {
  font-size: 1.25rem;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-md);
  flex-shrink: 0;
}

.activity-high .activity-icon {
  background: var(--color-error);
  color: var(--color-text-inverse);
}

.activity-medium .activity-icon {
  background: var(--color-warning);
  color: var(--color-text-primary);
}

.activity-low .activity-icon {
  background: var(--color-success);
  color: var(--color-text-inverse);
}

.activity-content {
  flex: 1;
  min-width: 0;
}

.activity-text {
  margin-bottom: var(--space-xs);
}

/* Responsive Design */
@media (max-width: 768px) {
  .hero-section {
    padding: var(--space-xl) var(--space-md);
    margin-bottom: var(--space-xl);
  }

  .hero-title {
    font-size: var(--text-3xl);
  }

  .hero-subtitle {
    font-size: var(--text-base);
  }

  .quick-action-icon {
    font-size: 2rem;
  }

  .hero-bg-circle {
    display: none;
  }

  .chat-action-content {
    flex-direction: column;
    text-align: center;
    gap: var(--space-sm);
  }

  .chat-action-arrow {
    display: none;
  }
}

@media (max-width: 480px) {
  .hero-section {
    padding: var(--space-lg) var(--space-sm);
  }

  .activity-item {
    padding: var(--space-sm);
  }

  .activity-icon {
    width: 32px;
    height: 32px;
    font-size: 1rem;
  }
}

/* Accessibility */
@media (prefers-reduced-motion: reduce) {
  .hero-bg-circle {
    animation: none;
  }

  .chat-action-arrow {
    transition: none;
  }
}
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_farm_stats_default() {
        let stats = FarmStats::default();
        assert_eq!(stats.total_diagnoses, 15);
        assert_eq!(stats.health_score, 92.0);
    }

    #[test]
    fn test_css_generation() {
        let css = generate_homepage_css();
        assert!(css.contains("hero-section"));
        assert!(css.contains("dashboard-section"));
        assert!(css.contains("@keyframes"));
    }
}
