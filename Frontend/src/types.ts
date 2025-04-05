export interface NewsArticle {
  id: string;
  title: string;
  source: string;
  type: string;
  summary: string;
  url: string;
  sentiment?: 'positive' | 'neutral' | 'negative';
}

export interface SearchFilters {
  dateRange: 'day' | 'week' | 'month' | 'year';
  sources: string[];
  sortBy: 'relevance' | 'date';
}

export interface CoinPrice {
  price: number;
  change24h: number;
  timestamp: string;
}