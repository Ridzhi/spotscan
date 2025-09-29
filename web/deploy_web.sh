npm run build && \
ssh dev rm -fr /var/www/html/spotscan && \
ssh dev mkdir /var/www/html/spotscan && \
scp -r dist/* dev:/var/www/html/spotscan
