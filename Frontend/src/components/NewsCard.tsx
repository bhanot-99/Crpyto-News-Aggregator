import { ExternalLink } from 'lucide-react';
import type { NewsArticle } from '../types';

interface NewsCardProps {
  article: NewsArticle;
}

export function NewsCard({ article }: NewsCardProps) {
  console.table(article);
  return (
    <article className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow">
      <div className="flex items-start gap-4">
        <img
          src={article.logo || `https://static.coinpaprika.com/coin/${article.id}/logo.png`}
          alt={`${article.name} logo`}
          className="w-16 h-16 rounded-full object-cover"
        //onError={(e) => (e.currentTarget.src = `https://static.coinpaprika.com/coin/${article.id}/logo.png`)}
        />
        <div>
          <h3 className="text-xl font-semibold mb-2 text-gray-900 dark:text-gray-100">
            {article.name}
          </h3>
          <p className="text-sm text-gray-600 dark:text-gray-400 mb-2">
            Symbol: {article.symbol}
          </p>
          <p className="text-sm text-gray-600 dark:text-gray-400">
            Started At: {article.started_at ? new Date(article.started_at).toLocaleDateString() : 'N/A'}
          </p>
        </div>
      </div>
      <p className="text-gray-700 dark:text-gray-300 mt-4 mb-4">
        {article.description || 'No description available.'}
      </p>
      <a
        href={`https://coinpaprika.com/coin/${article.symbol}/`}
        target="_blank"
        rel="noopener noreferrer"
        className="inline-flex items-center text-blue-600 dark:text-blue-400 hover:underline"
      >
        Read More
        <ExternalLink className="ml-1 w-4 h-4" />
      </a>
    </article>
  );
}