package misc;

import com.amazonaws.auth.AWSCredentials;
import com.amazonaws.auth.AWSStaticCredentialsProvider;
import com.amazonaws.auth.BasicAWSCredentials;
import com.amazonaws.regions.Regions;
import com.amazonaws.services.s3.AmazonS3;
import com.amazonaws.services.s3.AmazonS3ClientBuilder;
import com.amazonaws.services.s3.model.ListObjectsV2Request;
import com.amazonaws.services.s3.model.ListObjectsV2Result;

public class AwsExample {
    public static void main(String[] args) {
        doesFolderExist();
    }

    private static void doesFolderExist() {
        String bucket = "bay1-bkt-dev-us-east-1-test-firebolt-ingestion";
        String folder = "search/is_card/dev-test/2022/03/28/16/40/";
        AWSCredentials credentials = new BasicAWSCredentials(
                System.getenv("AWS_ACCESS_KEY"),
                System.getenv("AWS_SECRET_KEY")
        );

        AmazonS3 s3client = AmazonS3ClientBuilder
                .standard()
                .withCredentials(new AWSStaticCredentialsProvider(credentials))
                .withRegion(Regions.US_EAST_1)
                .build();

        System.out.println("doesObjectExist: " + s3client.doesObjectExist(bucket, folder));
        ListObjectsV2Request request = new ListObjectsV2Request()
                .withBucketName(bucket)
                .withPrefix(folder);
        ListObjectsV2Result result;
        result = s3client.listObjectsV2(request);
        // The folder itself counts as 1
        boolean exists = result.getKeyCount() > 0;
        System.out.println(exists);
    }
}
