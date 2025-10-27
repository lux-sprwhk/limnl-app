export type ZodiacSign =
	| 'aries'
	| 'taurus'
	| 'gemini'
	| 'cancer'
	| 'leo'
	| 'virgo'
	| 'libra'
	| 'scorpio'
	| 'sagittarius'
	| 'capricorn'
	| 'aquarius'
	| 'pisces';

export type MBTIType =
	| 'istj'
	| 'isfj'
	| 'infj'
	| 'intj'
	| 'istp'
	| 'isfp'
	| 'infp'
	| 'intp'
	| 'estp'
	| 'esfp'
	| 'enfp'
	| 'entp'
	| 'estj'
	| 'esfj'
	| 'enfj'
	| 'entj';

export interface UserProfile {
	name: string;
	zodiacSign?: ZodiacSign;
	mbtiType?: MBTIType;
}

export const ZODIAC_SIGNS: { value: ZodiacSign; label: string }[] = [
	{ value: 'aries', label: '♈ Aries' },
	{ value: 'taurus', label: '♉ Taurus' },
	{ value: 'gemini', label: '♊ Gemini' },
	{ value: 'cancer', label: '♋ Cancer' },
	{ value: 'leo', label: '♌ Leo' },
	{ value: 'virgo', label: '♍ Virgo' },
	{ value: 'libra', label: '♎ Libra' },
	{ value: 'scorpio', label: '♏ Scorpio' },
	{ value: 'sagittarius', label: '♐ Sagittarius' },
	{ value: 'capricorn', label: '♑ Capricorn' },
	{ value: 'aquarius', label: '♒ Aquarius' },
	{ value: 'pisces', label: '♓ Pisces' }
];

export const MBTI_TYPES: { value: MBTIType; label: string }[] = [
	{ value: 'istj', label: '📋 ISTJ - The Logistician' },
	{ value: 'isfj', label: '🛡️ ISFJ - The Defender' },
	{ value: 'infj', label: '🔮 INFJ - The Advocate' },
	{ value: 'intj', label: '🏗️ INTJ - The Architect' },
	{ value: 'istp', label: '🔧 ISTP - The Virtuoso' },
	{ value: 'isfp', label: '🎨 ISFP - The Adventurer' },
	{ value: 'infp', label: '💭 INFP - The Mediator' },
	{ value: 'intp', label: '🧠 INTP - The Logician' },
	{ value: 'estp', label: '⚡ ESTP - The Entrepreneur' },
	{ value: 'esfp', label: '🎭 ESFP - The Entertainer' },
	{ value: 'enfp', label: '🌟 ENFP - The Campaigner' },
	{ value: 'entp', label: '💡 ENTP - The Debater' },
	{ value: 'estj', label: '👔 ESTJ - The Executive' },
	{ value: 'esfj', label: '🤝 ESFJ - The Consul' },
	{ value: 'enfj', label: '🎤 ENFJ - The Protagonist' },
	{ value: 'entj', label: '👑 ENTJ - The Commander' }
];

export const DEFAULT_USER_PROFILE: UserProfile = {
	name: '',
	zodiacSign: undefined,
	mbtiType: undefined
};
