import { ExternalLink, ThumbsUp, Minus, ThumbsDown } from 'lucide-react';
import type { NewsArticle } from '../types';

interface NewsCardProps {
  article: NewsArticle;
}

export function NewsCard({ article }: NewsCardProps) {
  const sentimentIcon = {
    positive: <ThumbsUp className="w-4 h-4 text-green-500" />,
    neutral: <Minus className="w-4 h-4 text-gray-500" />,
    negative: <ThumbsDown className="w-4 h-4 text-red-500" />,
  };

  return (
    <article className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow">
      <div className="flex justify-between items-start gap-4">
        <div>
          <h3 className="text-xl font-semibold mb-2 text-gray-900 dark:text-gray-100">
            {article.title}
          </h3>
          <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">
            {article.source}
          </p>
        </div>
        {article.sentiment && (
          <div className="flex items-center gap-1">
            {sentimentIcon[article.sentiment]}
          </div>
        )}
      </div>
      <p className="text-gray-700 dark:text-gray-300 mb-4">{article.summary}</p>
      <a
        href={article.url}
        target="_blank"
        rel="noopener noreferrer"
        className="inline-flex items-center text-blue-600 dark:text-blue-400 hover:underline"
      >
        Read more
        <ExternalLink className="ml-1 w-4 h-4" />
      </a>
    </article>
  );
}