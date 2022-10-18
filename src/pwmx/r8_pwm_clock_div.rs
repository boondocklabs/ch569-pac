#[doc = "Register `R8_PWM_CLOCK_DIV` reader"]
pub struct R(crate::R<R8_PWM_CLOCK_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_PWM_CLOCK_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_PWM_CLOCK_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_PWM_CLOCK_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_PWM_CLOCK_DIV` writer"]
pub struct W(crate::W<R8_PWM_CLOCK_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_PWM_CLOCK_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<R8_PWM_CLOCK_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_PWM_CLOCK_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R8_PWM_CLOCK_DIV` reader - PWM clock divisor"]
pub type R8_PWM_CLOCK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `R8_PWM_CLOCK_DIV` writer - PWM clock divisor"]
pub type R8_PWM_CLOCK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, R8_PWM_CLOCK_DIV_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - PWM clock divisor"]
    #[inline(always)]
    pub fn r8_pwm_clock_div(&self) -> R8_PWM_CLOCK_DIV_R {
        R8_PWM_CLOCK_DIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - PWM clock divisor"]
    #[inline(always)]
    pub fn r8_pwm_clock_div(&mut self) -> R8_PWM_CLOCK_DIV_W<0> {
        R8_PWM_CLOCK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM clock divisor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_pwm_clock_div](index.html) module"]
pub struct R8_PWM_CLOCK_DIV_SPEC;
impl crate::RegisterSpec for R8_PWM_CLOCK_DIV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_pwm_clock_div::R](R) reader structure"]
impl crate::Readable for R8_PWM_CLOCK_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_pwm_clock_div::W](W) writer structure"]
impl crate::Writable for R8_PWM_CLOCK_DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_PWM_CLOCK_DIV to value 0"]
impl crate::Resettable for R8_PWM_CLOCK_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
